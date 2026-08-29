#![doc = include_str!("../README.md")]

use core::fmt;

/// Asserts a compile-time invariant.
///
/// # Examples
///
/// ```
/// use crustaceanstyle::invariant;
///
/// const FRAME_SIZE_MAX: u32 = 128 * 1024;
/// const MESSAGE_SIZE_MAX: u32 = 64 * 1024;
///
/// invariant!(MESSAGE_SIZE_MAX <= FRAME_SIZE_MAX);
/// ```
#[macro_export]
macro_rules! invariant {
    ($condition:expr $(,)?) => {
        const _: () = assert!($condition);
    };
}

/// A zero-based position in a sequence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Index(u32);

impl Index {
    /// Creates an index.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Converts this zero-based index to a one-based count.
    #[must_use]
    pub const fn to_count(self) -> Option<Count> {
        match self.0.checked_add(1) {
            Some(value) => Some(Count(value)),
            None => None,
        }
    }
}

/// A one-based quantity of items.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Count(u32);

impl Count {
    /// Creates a count.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A size measured in bytes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SizeBytes(u64);

impl SizeBytes {
    /// Creates a byte size.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the number of bytes.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for SizeBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{Count, Index, SizeBytes};

    #[test]
    fn index_converts_to_count() {
        assert_eq!(Index::new(0).to_count(), Some(Count::new(1)));
        assert_eq!(Index::new(41).to_count(), Some(Count::new(42)));
    }

    #[test]
    fn maximum_index_does_not_overflow() {
        assert_eq!(Index::new(u32::MAX).to_count(), None);
    }

    #[test]
    fn byte_size_preserves_value() {
        assert_eq!(SizeBytes::new(4096).get(), 4096);
        assert_eq!(SizeBytes::new(4096).to_string(), "4096");
    }
}
