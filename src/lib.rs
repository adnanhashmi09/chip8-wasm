//! CHIP-8 Emulator written in Rust, compiled to WebAssembly.
//!
//! This crate provides a Chip-8 CPU emulator that can be used in the browser
//! via WebAssembly.

pub mod memory;
pub mod fonts;
pub mod display;
pub mod cpu;
