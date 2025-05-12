use crate::engine::errors::EngineError;

pub trait ErrorTrait {
    fn to_string(&self) -> String;
}

pub enum Error {
    EngineError(EngineError),
}

impl ErrorTrait for Error {
    fn to_string(&self) -> String {
        match self {
            Error::EngineError(e) => e.to_string(),
        }
    }
}

impl From<EngineError> for Error {
    fn from(err: EngineError) -> Self {
        Error::EngineError(err)
    }
}
