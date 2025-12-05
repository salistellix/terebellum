pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// region:      --- Newtype Wrapper

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
