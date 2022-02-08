#[derive(Debug)]
pub enum Error<E> {
    ParserError,
    Aborted,
    IoError(E),
    EOF,
}

impl<E> From<E> for Error<E> {
    fn from(err: E) -> Self {
        Self::IoError(err)
    }
}