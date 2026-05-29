//! WASM bindings for the Chip-8 emulator.
//!
//! This module exports the Chip-8 emulator to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;

// Use chip8::Chip8 in WASM context
#[wasm_bindgen]
pub struct Chip8Wasm {
    chip8: crate::chip8::Chip8,
}

#[wasm_bindgen]
impl Chip8Wasm {
    /// Create a new Chip-8 emulator instance.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8Wasm {
        Chip8Wasm {
            chip8: crate::chip8::Chip8::new(),
        }
    }

    /// Reset the emulator to initial state.
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    /// Load a ROM from a byte array.
    pub fn load_rom(&mut self, data: &[u8]) -> Result<usize, JsValue> {
        self.chip8
            .load_rom(data)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Load a demo program.
    pub fn load_demo(&mut self) {
        self.chip8.load_demo();
    }

    /// Execute a single instruction.
    pub fn step(&mut self) {
        self.chip8.step();
    }

    /// Execute multiple instructions (one frame).
    pub fn update(&mut self) {
        self.chip8.update();
    }

    /// Get the display buffer (64x32 pixels as RGBA).
    pub fn get_display(&self) -> Vec<u8> {
        let pixels = self.chip8.get_display();
        let mut rgba = vec![0u8; pixels.len() * 4];
        
        for (i, &pixel) in pixels.iter().enumerate() {
            let offset = i * 4;
            if pixel == 1 {
                // White pixel
                rgba[offset] = 255;     // R
                rgba[offset + 1] = 255; // G
                rgba[offset + 2] = 255; // B
                rgba[offset + 3] = 255; // A
            }
            // Alpha stays 0 for black (transparency)
        }
        
        rgba
    }

    /// Press a key (0-15).
    pub fn key_press(&mut self, key: u8) {
        self.chip8.key_press(key);
    }

    /// Release a key (0-15).
    pub fn key_release(&mut self, key: u8) {
        self.chip8.key_release(key);
    }

    /// Set key from keyboard character.
    pub fn set_key(&mut self, keycode: char, pressed: bool) {
        self.chip8.set_key_from_keycode(keycode, pressed);
    }

    /// Check if a key is pressed.
    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.chip8.is_key_pressed(key)
    }

    /// Get delay timer value.
    pub fn get_delay_timer(&self) -> u8 {
        self.chip8.get_delay_timer()
    }

    /// Get sound timer value.
    pub fn get_sound_timer(&self) -> u8 {
        self.chip8.get_sound_timer()
    }

    /// Get program counter (for debugging).
    pub fn get_pc(&self) -> u16 {
        self.chip8.get_pc()
    }

    /// Get register value (0-15).
    pub fn get_register(&self, index: usize) -> u8 {
        self.chip8.get_register(index)
    }

    /// Get index register.
    pub fn get_index(&self) -> u16 {
        self.chip8.get_index()
    }

    /// Get display width.
    pub fn display_width() -> u32 {
        crate::display::DISPLAY_WIDTH as u32
    }

    /// Get display height.
    pub fn display_height() -> u32 {
        crate::display::DISPLAY_HEIGHT as u32
    }
}
