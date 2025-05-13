use crate::error::ErrorTrait;

pub enum InputError {
    OutOfBounds { index: u8, size: usize },
}

impl ErrorTrait for InputError {
    fn to_string(&self) -> String {
        match self {
            InputError::OutOfBounds { index, size } => {
                format!("Key index out of bounds: {} for size {}", index, size)
            },
        }
    }
}
