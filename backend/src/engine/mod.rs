use crate::display::{constants::FONT_SET, Display};
pub use crate::display::constants::{HEIGHT, WIDTH};
use crate::input::Input;


use constants::{MEMORY_SIZE, START_ADDRESS};
use errors::EngineError;
use random::MultiplyWithCarry;

mod constants;
pub mod errors;
mod random;

pub struct Engine {
    registers: [u8; 16],
    index: u16,
    pc: u16,
    memory: [u8; MEMORY_SIZE],
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    input: Input,
    display: Display,
    random: MultiplyWithCarry,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine = Self {
            registers: [0; 16],
            index: 0,
            pc: START_ADDRESS as u16,
            memory: [0; MEMORY_SIZE],
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            input: Input::new(),
            display: Display::new(),
            random: random::MultiplyWithCarry::new(42),
        };

        for (i, byte) in FONT_SET.iter().enumerate() {
            engine.memory[i] = *byte;
        }

        engine
    }

    fn decode_opcode(&mut self, opcode: u16) -> Result<(), EngineError> {
        let prefix: u8 = ((opcode & 0xF000) >> 12) as u8;
        let register_x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let register_y: u8 = ((opcode & 0x00F0) >> 4) as u8;
        let operation: u8 = (opcode & 0x000F) as u8;

        self.pc += 2;

        match prefix {
            0x0 => match (register_y, operation) {
                // 00E0 | CLS | Clears the screen
                (0xE, 0x0) => self.display.clear()?,
                // 00EE | RET | Returns from a subroutine
                (0xE, 0xE) => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                },

                _ => Err(EngineError::OpCodeNotFound {
                    op_code: opcode as u8,
                })?,
            },
            // 1NNN | JP | Jumps to address NNN
            0x1 => self.pc = opcode & 0x0FFF,
            // 2NNN | CALL | Calls subroutine at NNN
            0x2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            },
            // 3XNN | SE VX NN | Skips the next instruction if VX == NN
            0x3 => {
                if self.registers[register_x as usize] == (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            },
            // 4XNN | SNE VX NN | Skips the next instruction if VX != NN
            0x4 => {
                if self.registers[register_x as usize] != (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            },
            // 5XY0 | SE VX VY | Skips the next instruction if VX == VY
            0x5 => {
                if self.registers[register_x as usize] == self.registers[register_y as usize] {
                    self.pc += 2;
                }
            },
            // 6XNN | LD VX | Sets VX to NN
            0x6 => self.registers[register_x as usize] = (opcode & 0x00FF) as u8,
            // 7XNN | ADD VX, NN | Adds NN to VX
            0x7 => {
                let (result, overflow) =
                    self.registers[register_x as usize].overflowing_add((opcode & 0x00FF) as u8);
                self.registers[0xF] = if overflow { 1 } else { 0 };
                self.registers[register_x as usize] = result;
            },
            0x8 => match operation {
                // 8XY0 | LD VX, VY | Sets VX to the value of VY
                0x0 => self.registers[register_x as usize] = self.registers[register_y as usize],
                // 8XY1 | OR VX, VY | Sets VX to VX OR VY
                0x1 => self.registers[register_x as usize] |= self.registers[register_y as usize],
                // 8XY2 | AND VX, VY | Sets VX to VX AND VY
                0x2 => self.registers[register_x as usize] &= self.registers[register_y as usize],
                // 8XY3 | XOR VX, VY | Sets VX to VX XOR VY
                0x3 => self.registers[register_x as usize] ^= self.registers[register_y as usize],
                // 8XY4 | ADD VX, VY | Adds VY to VX
                0x4 => {
                    let (result, overflow) = self.registers[register_x as usize]
                        .overflowing_add(self.registers[register_y as usize]);
                    self.registers[0xF] = if overflow { 1 } else { 0 };
                    self.registers[register_x as usize] = result;
                },
                // 8XY5 | SUB VX, VY | Subtracts VY from VX
                0x5 => {
                    let (result, borrow) = self.registers[register_x as usize]
                        .overflowing_sub(self.registers[register_y as usize]);
                    self.registers[0xF] = if borrow { 0 } else { 1 };
                    self.registers[register_x as usize] = result;
                },
                // 8XY6 | SHR VX {, VY} | Shifts VX to the right by 1
                0x6 => {
                    self.registers[0xF] = self.registers[register_x as usize] & 0x01;
                    self.registers[register_x as usize] >>= 1;
                },
                // 8XY7 | SUBN VX, VY | Sets VX to VY minus VX
                0x7 => {
                    let (result, borrow) = self.registers[register_y as usize]
                        .overflowing_sub(self.registers[register_x as usize]);
                    self.registers[0xF] = if borrow { 0 } else { 1 };
                    self.registers[register_x as usize] = result;
                },
                // 8XYE | SHL VX {, VY} | Shifts VX to the left by 1
                0xE => {
                    // self.registers[0xF] = (self.registers[register_x as usize] & 0x80) >> 7; // TODO: Mirar a ver si es correcto
                    self.registers[0xF] = self.registers[register_x as usize] & 0x80;
                    self.registers[register_x as usize] <<= 1;
                },

                _ => Err(EngineError::OpCodeNotFound {
                    op_code: opcode as u8,
                })?,
            },
            // 9XY0 | SNE VX, VY | Skips the next instruction if VX != VY
            0x9 => {
                if self.registers[register_x as usize] != self.registers[register_y as usize] {
                    self.pc += 2;
                }
            },
            // ANNN | LD I, NNN | Sets I to the address NNN
            0xA => self.index = opcode & 0x0FFF,
            // BNNN | JP V0, NNN | Jumps to the address NNN + V0
            0xB => self.pc = (opcode & 0x0FFF) + self.registers[0] as u16,
            // CXNN | RND VX, NN | Sets VX to the result of a bitwise and operation on a random number and NN
            0xC => {
                self.registers[register_x as usize] =
                    self.random.random() as u8 & (opcode & 0x00FF) as u8
            },
            // DXYN | DRW VX, VY, N | Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels
            0xD => {
                let collision = self.display.draw(
                    self.registers[register_x as usize] as usize,
                    self.registers[register_y as usize] as usize,
                    &self.memory[self.index as usize..(self.index as usize + operation as usize)], // TODO: Mirar a ver si es correcto
                )?;

                self.registers[0xF] = if collision { 1 } else { 0 };
            },
            0xE => match (register_y, operation) {
                // EX9E | SKP VX | Skips the next instruction if the key stored in VX is pressed
                (0x9, 0xE) => {
                    if self.input.is_key_down(self.registers[register_x as usize])? {
                        self.pc += 2;
                    }
                },
                // EXA1 | SKNP VX | Skips the next instruction if the key stored in VX is not pressed
                (0xA, 0x1) => {
                    if !self.input.is_key_down(self.registers[register_x as usize])? {
                        self.pc += 2;
                    }
                },

                _ => Err(EngineError::OpCodeNotFound {
                    op_code: opcode as u8,
                })?,
            },
            0xF => match (register_y, operation) {
                // FX07 | LD VX, DT | Sets VX to the value of the delay timer
                (0x0, 0x7) => self.registers[register_x as usize] = self.delay_timer,
                // FX0A | LD VX, N | A key press is awaited, and then stored in VX
                (0x0, 0xA) => {
                    self.pc -= 2;

                    for (i, key) in self.input.keys.iter().enumerate() {
                        if *key {
                            self.registers[register_x as usize] = i as u8;
                            self.pc += 2;
                        }
                    }
                },
                // FX15 | LD DT, VX | Sets the delay timer to VX
                (0x1, 0x5) => self.delay_timer = self.registers[register_x as usize],
                // FX18 | LD ST, VX | Sets the sound timer to VX
                (0x1, 0x8) => self.sound_timer = self.registers[register_x as usize],
                // FX1E | ADD I, VX | Adds VX to I
                (0x1, 0xE) => self.index += self.registers[register_x as usize] as u16,
                // FX29 | LD F, VX | Sets I to the location of the sprite for the character in VX
                (0x2, 0x9) => self.index = self.registers[register_x as usize] as u16 * 5,
                // FX33 | LD B, VX | Stores the binary-coded decimal representation of VX in memory locations I, I+1, and I+2
                (0x3, 0x3) => {
                    self.memory[self.index as usize] = self.registers[register_x as usize] / 100;
                    self.memory[self.index as usize + 1] =
                        (self.registers[register_x as usize] / 10) % 10;
                    self.memory[self.index as usize + 2] =
                        (self.registers[register_x as usize] % 100) % 10;
                },
                // FX55 | LD [I], VX | Stores from V0 to VX in memory, starting at address I
                (0x5, 0x5) => self.memory
                    [(self.index as usize)..(self.index + register_x as u16 + 1) as usize]
                    .copy_from_slice(&self.registers[0..=register_x as usize]),
                // FX65 | LD VX, [I] | Fills from V0 to VX with values from memory, starting at address I
                (0x6, 0x5) => self.registers[0..(register_x as usize + 1)].copy_from_slice(
                    &self.memory
                        [self.index as usize..(self.index + register_x as u16 + 1) as usize],
                ),

                _ => Err(EngineError::OpCodeNotFound {
                    op_code: opcode as u8,
                })?,
            },

            _ => Err(EngineError::OpCodeNotFound {
                op_code: opcode as u8,
            })?,
        }

        Ok(())
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), EngineError> {
        if rom_data.len() > MEMORY_SIZE - START_ADDRESS {
            Err(EngineError::RomTooLarge {
                size: rom_data.len(),
            })?;
        }

        *self = Self::new();

        self.memory[START_ADDRESS..(START_ADDRESS + rom_data.len())].copy_from_slice(rom_data);

        Ok(())
    }

    pub fn execute_cycle(&mut self) -> Result<(), EngineError> {
        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc + 1) as usize] as u16);

        self.decode_opcode(opcode)?;

        Ok(())
    }

    pub fn decrement_timer(&mut self) -> Result<(), EngineError> {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        Ok(())
    }

    pub fn get_display(&self) -> &[u8; WIDTH * HEIGHT] {
        self.display.get_memory()
    }

    pub fn is_sound_active(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn key_down(&mut self, key: u8) -> Result<(), EngineError> {
        self.input.key_down(key)?;

        Ok(())
    }

    pub fn key_up(&mut self, key: u8) -> Result<(), EngineError> {
        self.input.key_up(key)?;

        Ok(())
    }
}
