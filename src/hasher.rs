use generic_array::{ArrayLength, GenericArray};

/// Stack allocated digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Digest<Size: ArrayLength<u8> + core::fmt::Debug + Eq + Send + Sync + 'static>(
    GenericArray<u8, Size>,
);

impl<Size: ArrayLength<u8> + core::fmt::Debug + Eq + Send + Sync + 'static> Digest<Size> {
    /// Creates a new digest from an array.
    pub fn new(digest: GenericArray<u8, Size>) -> Self {
        Self(digest)
    }
}

impl<Size: ArrayLength<u8> + core::fmt::Debug + Eq + Send + Sync + 'static> AsRef<[u8]>
    for Digest<Size>
{
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Trait implemented by a hash function implementation.
pub trait Hasher: Default {
    /// Digest size.
    type Size: ArrayLength<u8> + core::fmt::Debug + Eq + Send + Sync + 'static;

    /// Consume input and update internal state.
    fn update(&mut self, input: &[u8]);

    /// Returns the internal state digest.
    fn finalize(&self) -> Digest<Self::Size>;

    /// Reset the internal hasher state.
    fn reset(&mut self);

    /// Returns the digest of the input.
    fn digest(input: &[u8]) -> Digest<Self::Size>
    where
        Self: Sized,
    {
        let mut hasher = Self::default();
        hasher.update(input);
        hasher.finalize()
    }
}

/// New type wrapper for a hasher that implements the `std::io::Write` trait.
#[cfg(feature = "std")]
pub struct WriteHasher<H: Hasher>(H);

#[cfg(feature = "std")]
impl<H: Hasher> std::io::Write for WriteHasher<H> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
