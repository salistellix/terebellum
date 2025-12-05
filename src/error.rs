#[derive(derive_more::From, Debug)]
pub enum Error {
    /// A generic catch-all error when no specific variant exists yet.
    #[from]
    Generic(String),
}

// region:     --- Generic

impl Error {
    pub fn generic(value: impl core::fmt::Display) -> Self {
        Self::Generic(value.to_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Generic(value.to_string())
    }
}

// endregion:  --- Generic

// region:     --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl core::error::Error for Error {}

// endregion:  --- Error Boilerplate
