use crate::{OWNED_FLAG, OxStr, ReserveError};
use alloc::alloc::{alloc, dealloc, realloc};
use core::cmp::max;
use core::fmt;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr::{NonNull, copy_nonoverlapping};
use core::sync::atomic::AtomicUsize;

/// A buffer for incrementally building an owned [`OxStr`].
///
/// `OxStrBuilder` can be used when the final string is assembled from several
/// pieces. Convert it into an [`OxStr`] when construction is complete. The
/// conversion transfers ownership of the builder's buffer.
///
/// ```
/// use oxstr::{OxStr, OxStrBuilder};
///
/// let mut builder = OxStrBuilder::new();
/// builder.push_str("hello");
/// builder.push(' ');
/// builder.push_str("world");
///
/// let value = OxStr::from(builder);
/// assert_eq!(value, "hello world");
/// ```
pub struct OxStrBuilder {
    len: usize,
    data: NonNull<u8>,
}

impl OxStrBuilder {
    /// Creates a new empty `OxStrBuilder`.
    ///
    /// The builder initially has a capacity of zero bytes. If the amount of
    /// data to be appended is known in advance, consider using
    /// [`with_capacity`](Self::with_capacity) to avoid reallocations.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder};
    ///
    /// let builder = OxStrBuilder::new();
    /// assert_eq!(builder.capacity(), 0);
    /// assert_eq!(OxStr::from(builder), "");
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates a new empty `OxStrBuilder` with at least the specified capacity.
    ///
    /// The capacity is measured in bytes and can be queried with
    /// [`capacity`](Self::capacity). Reserving capacity in advance can reduce
    /// the number of reallocations needed while building a string.
    ///
    /// Panics if the requested capacity overflows or if the allocator reports
    /// a failure. Use [`try_with_capacity`](Self::try_with_capacity) for a
    /// fallible alternative.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder};
    ///
    /// let mut builder = OxStrBuilder::with_capacity(5);
    /// assert!(builder.capacity() >= 5);
    /// builder.push_str("hello");
    ///
    /// assert_eq!(OxStr::from(builder), "hello");
    /// ```
    #[inline]
    #[track_caller]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::try_with_capacity(capacity).unwrap_or_else(|e| e.unwrap())
    }

    /// Tries to create a new empty `OxStrBuilder` with at least the specified
    /// capacity.
    ///
    /// The capacity is measured in bytes.
    ///
    /// Returns an error if the requested capacity overflows or if the allocator
    /// reports a failure.
    ///
    /// ```
    /// use oxstr::OxStrBuilder;
    ///
    /// let builder = OxStrBuilder::try_with_capacity(16)?;
    /// assert!(builder.capacity() >= 16);
    /// # Ok::<(), oxstr::ReserveError>(())
    /// ```
    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<Self, ReserveError> {
        // SAFETY: we carefully choose the layout. Then we can allocate, check that allocation works and write to the allocation
        unsafe {
            let layout = OxStr::owned_layout_for_len(capacity)
                .map_err(|_| ReserveError::CapacityOverflow)?;
            let data = NonNull::new(alloc(layout)).ok_or(ReserveError::AllocError {
                layout,
                non_exhaustive: (),
            })?;
            data.cast::<usize>().write(capacity);
            Ok(Self { len: 0, data })
        }
    }

    /// Reserves capacity for at least `additional` bytes more than the current
    /// length.
    ///
    /// The allocator may reserve more space to avoid frequent reallocations.
    /// This method does nothing if the existing capacity is sufficient.
    ///
    /// Panics if the new capacity overflows or if the allocator reports a
    /// failure. Use [`try_reserve`](Self::try_reserve) for a fallible
    /// alternative.
    ///
    /// ```
    /// use oxstr::OxStrBuilder;
    ///
    /// let mut builder = OxStrBuilder::new();
    /// builder.reserve(10);
    /// assert!(builder.capacity() >= 10);
    /// ```
    #[inline]
    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.try_reserve(additional).unwrap_or_else(|e| e.unwrap())
    }

    /// Tries to reserve capacity for at least `additional` bytes more than the
    /// current length.
    ///
    /// The allocator may reserve more space to avoid frequent reallocations.
    /// This method does nothing if the existing capacity is sufficient. The
    /// builder's contents are preserved if an error occurs.
    ///
    /// Returns an error if the new capacity overflows or if the allocator
    /// reports a failure.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder, ReserveError};
    ///
    /// let mut builder = OxStrBuilder::new();
    /// builder.try_reserve(10)?;
    /// assert!(builder.capacity() >= 10);
    ///
    /// # Ok::<(), ReserveError>(())
    /// ```
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), ReserveError> {
        let current_capacity = self.capacity();
        let expected_capacity = self
            .len
            .checked_add(additional)
            .ok_or(ReserveError::CapacityOverflow)?;
        if current_capacity >= expected_capacity {
            return Ok(()); // We are fine
        }
        let expected_capacity = max(expected_capacity, current_capacity * 2);
        unsafe {
            self.data = do_realloc(current_capacity, expected_capacity, self.data)?;
            self.data.cast::<usize>().write(expected_capacity);
        };
        Ok(())
    }

    /// Appends a string slice to the end of this builder.
    ///
    /// Panics if the new capacity overflows or if the allocator reports a
    /// failure.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder};
    ///
    /// let mut builder = OxStrBuilder::new();
    /// builder.push_str("foo");
    /// builder.push_str("bar");
    ///
    /// assert_eq!(OxStr::from(builder), "foobar");
    /// ```
    #[inline]
    #[track_caller]
    pub fn push_str(&mut self, string: &str) {
        self.try_push_str(string).unwrap_or_else(|e| e.unwrap())
    }

    /// Appends a string slice to the end of this builder.
    ///
    /// Returns an error if the new capacity overflows or if the allocator
    /// reports a failure. The builder's contents are preserved if an error
    /// occurs.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder, ReserveError};
    ///
    /// let mut builder = OxStrBuilder::new();
    /// builder.try_push_str("🦀")?;
    ///
    /// assert_eq!(OxStr::from(builder), "🦀");
    /// # Ok::<(), ReserveError>(())
    /// ```
    #[inline]
    pub fn try_push_str(&mut self, string: &str) -> Result<(), ReserveError> {
        self.try_reserve(string.len())?;
        unsafe {
            copy_nonoverlapping(
                string.as_ptr(),
                self.data
                    .cast::<usize>()
                    .add(1)
                    .cast::<u8>()
                    .add(self.len)
                    .as_ptr(),
                string.len(),
            );
        }
        self.len += string.len();
        Ok(())
    }

    /// Appends the given [`char`] to the end of this builder.
    ///
    /// Panics if the new capacity overflows or if the allocator reports a
    /// failure.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder};
    ///
    /// let mut builder = OxStrBuilder::new();
    /// builder.push('a');
    /// builder.push('é');
    ///
    /// assert_eq!(OxStr::from(builder), "aé");
    /// ```
    #[inline]
    #[track_caller]
    pub fn push(&mut self, ch: char) {
        self.try_push(ch).unwrap_or_else(|e| e.unwrap())
    }

    /// Tries to append the given [`char`] to the end of this builder.
    ///
    /// Returns an error if the new capacity overflows or if the allocator
    /// reports a failure. The builder's contents are preserved if an error
    /// occurs.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder, ReserveError};
    ///
    /// let mut builder = OxStrBuilder::new();
    /// builder.try_push('🦀')?;
    ///
    /// assert_eq!(OxStr::from(builder), "🦀");
    /// # Ok::<(), ReserveError>(())
    /// ```
    #[inline]
    pub fn try_push(&mut self, ch: char) -> Result<(), ReserveError> {
        self.try_push_str(ch.encode_utf8(&mut [0; 4]))
    }

    /// Returns this builder's capacity, in bytes.
    ///
    /// ```
    /// use oxstr::OxStrBuilder;
    ///
    /// let builder = OxStrBuilder::with_capacity(10);
    /// assert!(builder.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { *self.data.cast::<usize>().as_ref() }
    }

    /// Removes all contents from this builder without changing its capacity.
    ///
    /// ```
    /// use oxstr::{OxStr, OxStrBuilder};
    ///
    /// let mut builder = OxStrBuilder::with_capacity(8);
    /// builder.push_str("discard");
    /// let capacity = builder.capacity();
    /// builder.clear();
    ///
    /// assert_eq!(builder.capacity(), capacity);
    /// assert_eq!(OxStr::from(builder), "");
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

// SAFETY: The type is not Clone so this is a move
unsafe impl Send for OxStrBuilder {}

// SAFETY: Mutable operations take &mut so their can't be any concurrent edit operation
unsafe impl Sync for OxStrBuilder {}

impl Drop for OxStrBuilder {
    fn drop(&mut self) {
        // SAFETY: the type is not Clone so we owned the buffer
        unsafe {
            #[expect(clippy::expect_used)]
            dealloc(
                self.data.as_ptr(),
                OxStr::owned_layout_for_len(self.capacity())
                    .expect("We have allocated with this layout"),
            );
        }
    }
}

/// Converts a builder into an owned [`OxStr`].
///
/// The builder's buffer is transferred to the returned string and any excess
/// capacity is released.
impl From<OxStrBuilder> for OxStr<'_> {
    #[inline]
    #[track_caller]
    fn from(mut value: OxStrBuilder) -> Self {
        let current_capacity = value.capacity();
        if current_capacity > value.len {
            // We shrink to fit
            unsafe {
                value.data = do_realloc(current_capacity, value.len, value.data)
                    .unwrap_or_else(|e| e.unwrap());
            }
        }
        let value = ManuallyDrop::new(value); // We are moving the implementation and don't want to run Drop
        unsafe {
            value.data.cast::<AtomicUsize>().write(AtomicUsize::new(1)); // We convert the capacity into a reference counter
        }
        OxStr {
            len: value.len | OWNED_FLAG,
            data: value.data,
            _marker: PhantomData,
        }
    }
}

impl Default for OxStrBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for OxStrBuilder {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.try_push_str(s).map_err(|_| fmt::Error)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.try_push(c).map_err(|_| fmt::Error)
    }
}

#[inline]
unsafe fn do_realloc(
    from_size: usize,
    to_size: usize,
    data: NonNull<u8>,
) -> Result<NonNull<u8>, ReserveError> {
    unsafe {
        let from_layout =
            OxStr::owned_layout_for_len(from_size).map_err(|_| ReserveError::CapacityOverflow)?;
        let to_layout =
            OxStr::owned_layout_for_len(to_size).map_err(|_| ReserveError::CapacityOverflow)?;
        NonNull::new(realloc(data.as_ptr(), from_layout, to_layout.size())).ok_or(
            ReserveError::AllocError {
                layout: to_layout,
                non_exhaustive: (),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Write;

    #[test]
    fn new_and_default_have_zero_capacity_and_contains_empty_string() {
        assert_eq!(OxStrBuilder::new().capacity(), 0);
        assert_eq!(OxStrBuilder::default().capacity(), 0);
        assert_eq!(OxStr::from(OxStrBuilder::new()), "");
        assert_eq!(OxStr::from(OxStrBuilder::default()), "");
    }

    #[test]
    fn with_capacity_reserves_requested_space() {
        let mut builder = OxStrBuilder::with_capacity(8);

        assert_eq!(builder.capacity(), 8);
        builder.reserve(8);
        assert_eq!(builder.capacity(), 8);
    }

    #[test]
    fn reserve_grows_capacity_geometrically() {
        let mut builder = OxStrBuilder::with_capacity(4);
        builder.push_str("hello");

        assert_eq!(builder.capacity(), 8);
        assert_eq!(OxStr::from(builder), "hello");
    }

    #[test]
    fn pushes_strings_and_unicode_characters() {
        let mut builder = OxStrBuilder::new();
        builder.push_str("hello");
        builder.push(' ');
        builder.push('🦀');

        assert_eq!(OxStr::from(builder), "hello 🦀");
    }

    #[test]
    fn clear_keeps_capacity_and_allows_reuse() {
        let mut builder = OxStrBuilder::with_capacity(16);
        builder.push_str("discarded");
        builder.clear();

        assert_eq!(builder.capacity(), 16);
        builder.push_str("kept");
        assert_eq!(OxStr::from(builder), "kept");
    }

    #[test]
    fn implements_fmt_write() {
        let mut builder = OxStrBuilder::new();
        #[expect(clippy::unwrap_used)]
        write!(&mut builder, "{} + {} = {}", 1, 2, 3).unwrap();

        assert_eq!(OxStr::from(builder), "1 + 2 = 3");
    }

    #[test]
    fn converts_empty_builder_to_owned_string() {
        let mut value = OxStr::from(OxStrBuilder::new());

        assert_eq!(value, "");
        assert!(value.get_mut().is_some());
    }

    #[test]
    fn try_reserve_reports_length_overflow() {
        assert_eq!(
            OxStrBuilder::new().try_reserve(usize::MAX),
            Err(ReserveError::CapacityOverflow)
        );

        let mut builder = OxStrBuilder::new();
        builder.push('a');
        assert_eq!(
            builder.try_reserve(usize::MAX),
            Err(ReserveError::CapacityOverflow)
        );
        assert_eq!(OxStr::from(builder), "a");
    }

    #[test]
    fn try_with_capacity_reports_length_overflow() {
        assert_eq!(
            OxStrBuilder::try_with_capacity(usize::MAX).map(|_| ()),
            Err(ReserveError::CapacityOverflow)
        );
    }
}
