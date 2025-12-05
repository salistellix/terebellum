pub use crate::error::Error;

/// Convenient alias tying the crate's custom [`Error`] type to `core::result::Result`.
///
/// Use this to keep error handling consistent across the crate without repeatedly
/// specifying the error variant.
pub type Result<T> = core::result::Result<T, Error>;

// region:      --- Newtype Wrapper

/// Lightweight newtype wrapper used to leverage `Deref`/`DerefMut` while keeping
/// the wrapped value strongly typed within the *Newtype Wrapper* region.
pub struct W<T>(pub T);

impl<T> core::ops::Deref for W<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for W<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// endregion:   --- Newtype Wrapper
