use crate::error::ErrorTrait;

pub enum DisplayError {
    OutOfBounds { x: u8, y: u8, width: u8, height: u8 },
}

impl ErrorTrait for DisplayError {
    fn to_string(&self) -> String {
        match self {
            DisplayError::OutOfBounds {
                x,
                y,
                width,
                height,
            } => {
                format!(
                    "Coordinates out of bounds: ({}, {}) for display size ({}, {})",
                    x, y, width, height
                )
            },
        }
    }
}
