//! CHIP-8 Emulator written in Rust, compiled to WebAssembly.
//!
//! This crate provides a Chip-8 CPU emulator that can be used in the browser
//! via WebAssembly.

pub mod memory;
pub mod fonts;
pub mod display;

// Re-export commonly used items
pub use memory::{Memory, RAM_SIZE, ROM_LOAD_ADDRESS, FONT_START};
pub use fonts::{FONT_SPRITES, FONT_SIZE, SPRITE_WIDTH, SPRITE_HEIGHT};
pub use display::{Display, DISPLAY_WIDTH, DISPLAY_HEIGHT, DISPLAY_SIZE};
