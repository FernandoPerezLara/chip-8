use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use engine::{Engine, HEIGHT, WIDTH};
use error::ErrorTrait;

mod display;
mod engine;
mod error;
mod input;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Chip8 {
    engine: Engine
}

#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: Engine::new()
        }
    }

    pub fn get_width(&self) -> usize {
        WIDTH
    }

    pub fn get_height(&self) -> usize {
        HEIGHT
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), JsError> {
        if let Err(e) = self.engine.load_rom(rom_data) {
            return Err(JsError::new(&e.to_string()));
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn execute_cycle(&mut self) -> Result<(), JsError> {
        if let Err(e) = self.engine.execute_cycle() {
            return Err(JsError::new(&e.to_string()));
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn decrement_timer(&mut self) -> Result<(), JsError> {
        if let Err(e) = self.engine.decrement_timer() {
            return Err(JsError::new(&e.to_string()));
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_display(&self) -> Vec<u8> {
        self.engine.get_display().to_vec()
    }

    #[wasm_bindgen]
    pub fn is_sound_active(&self) -> bool {
        self.engine.is_sound_active()
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: u8) -> Result<(), JsError> {
        if let Err(e) = self.engine.key_down(key) {
            return Err(JsError::new(&e.to_string()));
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn key_up(&mut self, key: u8) -> Result<(), JsError> {
        if let Err(e) = self.engine.key_up(key) {
            return Err(JsError::new(&e.to_string()));
        }

        Ok(())
    }
}
