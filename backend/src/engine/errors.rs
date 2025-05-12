use crate::display::errors::DisplayError;
use crate::error::ErrorTrait;
use crate::input::errors::InputError;

pub enum EngineError {
    RomTooLarge { size: usize },
    OpCodeNotFound { op_code: u8 },

    DisplayError(DisplayError),
    InputError(InputError),
}

impl ErrorTrait for EngineError {
    fn to_string(&self) -> String {
        match self {
            EngineError::RomTooLarge { size } => {
                format!("ROM size {} exceeds maximum allowed size", size)
            },
            EngineError::OpCodeNotFound { op_code } => {
                format!("OpCode {:#06X} not found", op_code)
            },

            EngineError::DisplayError(e) => e.to_string(),
            EngineError::InputError(e) => e.to_string(),
        }
    }
}

impl From<DisplayError> for EngineError {
    fn from(err: DisplayError) -> Self {
        EngineError::DisplayError(err)
    }
}

impl From<InputError> for EngineError {
    fn from(err: InputError) -> Self {
        EngineError::InputError(err)
    }
}
