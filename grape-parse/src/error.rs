#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    Unexpected,

    // TODO: migrate from Result<Option<T>, Error> to Result<T, Error>
    Continue,
}
