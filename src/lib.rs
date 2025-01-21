use std::fmt::Debug;

pub struct DispError<E: std::error::Error>
{
    error: E,
}

impl<E: std::error::Error> Debug for DispError<E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl<E: std::error::Error> From<E> for DispError<E>
{
    fn from(error: E) -> Self {
        Self { error }
    }
}
