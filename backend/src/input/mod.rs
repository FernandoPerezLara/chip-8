use constants::KEY_COUNT;
use errors::InputError;

mod constants;
pub mod errors;

pub struct Input {
    pub keys: [bool; KEY_COUNT],
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: [false; KEY_COUNT],
        }
    }

    pub fn key_down(&mut self, index: u8) -> Result<(), InputError> {
        if index >= KEY_COUNT as u8 {
            Err(InputError::OutOfBounds {
                index,
                size: KEY_COUNT,
            })?;
        }

        self.keys[index as usize] = true;

        Ok(())
    }

    pub fn key_up(&mut self, index: u8) -> Result<(), InputError> {
        if index >= KEY_COUNT as u8 {
            Err(InputError::OutOfBounds {
                index,
                size: KEY_COUNT,
            })?;
        }

        self.keys[index as usize] = false;

        Ok(())
    }

    pub fn is_key_down(&self, index: u8) -> Result<bool, InputError> {
        if index >= KEY_COUNT as u8 {
            Err(InputError::OutOfBounds {
                index,
                size: KEY_COUNT,
            })?;
        }

        Ok(self.keys[index as usize])
    }
}
