#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

/// A wrapper for errors implementing `std::error::Error`, that when `Debug`ged, will `Display` the contained error. Usually used in the signature of a `main` function - see examples in the [crate documentation](crate).
///
/// # Example
///
/// ```rust
/// # use disperror::DispError;
/// use std::io::{Error, ErrorKind};
/// let error = Error::new(ErrorKind::NotFound, "File not found");
/// let disp_error = DispError::from(error);
/// assert_eq!(format!("{:?}", disp_error), "File not found");
/// ```
pub struct DispError<E: Error>(E);

impl<E: Error> Debug for DispError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<E: Error> From<E> for DispError<E> {
    fn from(error: E) -> Self {
        Self(error)
    }
}
