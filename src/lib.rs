//! CHIP-8 Emulator written in Rust, compiled to WebAssembly.
//!
//! This crate provides a Chip-8 CPU emulator that can be used in the browser
//! via WebAssembly.

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod memory;
pub mod fonts;
pub mod display;
pub mod cpu;
pub mod keypad;
pub mod timers;
pub mod chip8;

// Re-exports for convenience
pub use memory::{Memory, RAM_SIZE, ROM_LOAD_ADDRESS};
pub use fonts::{FONT_SPRITES, FONT_SIZE, SPRITE_WIDTH, SPRITE_HEIGHT};
pub use display::{Display, DISPLAY_WIDTH, DISPLAY_HEIGHT, DISPLAY_SIZE};
pub use cpu::Cpu;
pub use chip8::Chip8;
