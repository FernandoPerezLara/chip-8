use constants::{HEIGHT, WIDTH};
use errors::DisplayError;

pub mod constants;
pub mod errors;

pub struct Display {
    memory: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            memory: [0; WIDTH * HEIGHT],
        }
    }

    pub fn get_memory(&self) -> &[u8; WIDTH * HEIGHT] {
        &self.memory
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), DisplayError> {
        if x >= WIDTH || y >= HEIGHT {
            Err(DisplayError::OutOfBounds {
                x: x as u8,
                y: y as u8,
                width: WIDTH as u8,
                height: HEIGHT as u8,
            })?;
        }

        self.memory[x + y * WIDTH] = value as u8;

        Ok(())
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<u8, DisplayError> {
        if x >= WIDTH || y >= HEIGHT {
            Err(DisplayError::OutOfBounds {
                x: x as u8,
                y: y as u8,
                width: WIDTH as u8,
                height: HEIGHT as u8,
            })?;
        }

        Ok(self.memory[x + y * WIDTH])
    }

    pub fn clear(&mut self) -> Result<(), DisplayError> {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, false)?;
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> Result<bool, DisplayError> {
        let mut collision = false;

        for (j, row) in sprite.iter().enumerate() {
            for i in 0..8 {
                let new_value = (row >> (7 - i)) & 0x01;

                if new_value == 1 {
                    let xi = (x + i) % WIDTH;
                    let yj = (y + j) % HEIGHT;

                    let old_value = self.get_pixel(xi, yj)?;

                    collision = collision || (old_value == 1);

                    self.set_pixel(xi, yj, (new_value == 1) ^ (old_value == 1))?;
                }
            }
        }

        Ok(collision)
    }
}
