//! Chip8 emulator - wraps all modules into a complete emulator.

use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use crate::memory::{Memory, ROM_LOAD_ADDRESS};
use crate::timers::Timers;
use crate::cpu::execute_instruction;
use crate::cpu::INSTRUCTIONS_PER_FRAME;

/// The complete Chip-8 emulator
#[derive(Debug)]
pub struct Chip8 {
    pub cpu: Cpu,
    pub memory: Memory,
    pub display: Display,
    keypad: Keypad,
    pub timers: Timers,
    /// Whether to continue running
    running: bool,
}

impl Chip8 {
    /// Create a new Chip-8 emulator.
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
            timers: Timers::new(),
            running: true,
        };
        // Load fonts into memory
        chip8.memory.load_fonts();
        chip8
    }

    /// Reset the entire system.
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
        self.memory.load_fonts(); // Reload fonts after reset!
        self.display.clear();
        self.keypad.reset();
        self.timers.reset();
        self.running = true;
        self.cpu.set_pc(0x200);
    }

    /// Load a ROM into memory.
    pub fn load_rom(&mut self, rom: &[u8]) -> Result<usize, String> {
        if rom.len() > 0xDFFF - 0x200 {
            return Err(format!("ROM too large: {} bytes", rom.len()));
        }
        Ok(self.memory.load_rom(rom))
    }

    /// Load a simple test program that draws "0" on screen.
    pub fn load_demo(&mut self) {
        // SIMPLE demo: Just clear and draw one sprite, then halt
        // Format: [high-byte, low-byte] for each opcode
        // PC starts at 0x200
        
        let demo = vec![
            // 0x200: CLS (00E0)
            0x00, 0xE0,
            // 0x202: LD V0, 10 (X position) - 6010
            0x60, 0x10,
            // 0x204: LD V1, 10 (Y position) - 6110
            0x61, 0x10,
            // 0x206: LD I, 0x000 (font '0') - A000
            0xA0, 0x00,
            // 0x208: DRW V0, V1, 5 - D015
            0xD0, 0x15,
            // 0x20A: JP 0x20A (halt forever)
            0x12, 0x0A,
        ];
        
        // Load the demo program
        for (i, &byte) in demo.iter().enumerate() {
            self.memory.write(ROM_LOAD_ADDRESS + i as u16, byte);
        }
        self.cpu.set_pc(ROM_LOAD_ADDRESS);
    }

    /// Execute a single instruction.
    pub fn step(&mut self) {
        execute_instruction(
            &mut self.cpu,
            &mut self.memory,
            &mut self.display,
            &self.keypad,
        );
    }

    /// Execute multiple instructions (one frame).
    pub fn update(&mut self) {
        for _ in 0..INSTRUCTIONS_PER_FRAME {
            self.step();
        }
        self.timers.tick();
    }

    /// Get the display buffer.
    pub fn get_display(&self) -> &[u8] {
        self.display.get_pixels()
    }

    /// Press a key.
    pub fn key_press(&mut self, key: u8) {
        self.keypad.press(key);
    }

    /// Release a key.
    pub fn key_release(&mut self, key: u8) {
        self.keypad.release(key);
    }

    /// Set key from keyboard character.
    pub fn set_key_from_keycode(&mut self, keycode: char, pressed: bool) {
        self.keypad.set_from_keycode(keycode, pressed);
    }

    /// Check if keypad key is pressed.
    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keypad.is_pressed(key)
    }

    /// Get delay timer.
    pub fn get_delay_timer(&self) -> u8 {
        self.timers.get_delay()
    }

    /// Get sound timer.
    pub fn get_sound_timer(&self) -> u8 {
        self.timers.get_sound()
    }

    /// Set delay timer.
    pub fn set_delay_timer(&mut self, value: u8) {
        self.timers.set_delay(value);
    }

    /// Set sound timer.
    pub fn set_sound_timer(&mut self, value: u8) {
        self.timers.set_sound(value);
    }

    /// Check if drawing occurred.
    pub fn needs_draw(&self) -> bool {
        self.cpu.draw_flag
    }

    /// Clear the draw flag.
    pub fn clear_draw_flag(&mut self) {
        self.cpu.draw_flag = false;
    }

    /// Get program counter (for debugging).
    pub fn get_pc(&self) -> u16 {
        self.cpu.get_pc()
    }

    /// Get register value.
    pub fn get_register(&self, index: usize) -> u8 {
        self.cpu.get_v(index)
    }

    /// Get index register.
    pub fn get_index(&self) -> u16 {
        self.cpu.get_i()
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
