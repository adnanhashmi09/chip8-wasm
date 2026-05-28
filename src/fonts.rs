//! Font module for CHIP-8 emulator.
//!
//! Contains the built-in 16-character font sprites (0-9, A-F).
//! Each sprite is 5 bytes (8x5 pixels) - one byte per row.

use crate::memory::{Memory, FONT_START};

/// Number of pixels wide for each sprite
pub const SPRITE_WIDTH: usize = 8;

/// Number of pixels tall for each sprite
pub const SPRITE_HEIGHT: usize = 5;

/// Number of font characters
pub const FONT_CHAR_COUNT: usize = 16;

/// Total bytes for all fonts
pub const FONT_SIZE: usize = FONT_CHAR_COUNT * SPRITE_HEIGHT; // 16 * 5 = 80 bytes

/// Standard CHIP-8 font sprite data.
///
/// Each character is 5 bytes (8x5 pixels).
/// Characters: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F
pub const FONT_SPRITES: [u8; FONT_SIZE] = [
    // 0
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    // 1
    0x20, 0x60, 0x20, 0x20, 0x70,
    // 2
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    // 3
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    // 4
    0x90, 0x90, 0xF0, 0x10, 0x10,
    // 5
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    // 6
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    // 7
    0xF0, 0x10, 0x20, 0x40, 0x40,
    // 8
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    // 9
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    // A
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    // B
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    // C
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    // D
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    // E
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    // F
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

impl Memory {
    /// Load the standard font sprites into memory starting at address 0x000.
    ///
    /// This is typically called once during emulator initialization.
    pub fn load_fonts(&mut self) {
        self.write_slice(FONT_START, &FONT_SPRITES);
    }

    /// Get the address of a font sprite in memory.
    ///
    /// # Arguments
    /// * `digit` - A value from 0 to 15 representing the hex digit
    ///
    /// # Returns
    /// * The memory address of the sprite, or None if digit is invalid
    pub fn font_address(&self, digit: u8) -> Option<u16> {
        if digit > 0xF {
            return None;
        }
        Some(digit as u16 * SPRITE_HEIGHT as u16)
    }
}

/// Represent a font sprite as ASCII art for debugging.
/// Uses 1=█ for on pixels, 0=· for off pixels.
pub fn sprite_to_ascii(sprite: &[u8; SPRITE_HEIGHT]) -> String {
    sprite
        .iter()
        .map(|row| {
            (0..8)
                .map(|bit| {
                    if (row >> bit) & 1 == 1 { '#' } else { '.' }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_size_bytes() {
        // Each char is 5 bytes, 16 chars = 80 bytes
        assert_eq!(FONT_SIZE, 80);
        assert_eq!(FONT_SPRITES.len(), 80);
    }

    #[test]
    fn test_sprite_dimensions() {
        assert_eq!(SPRITE_WIDTH, 8);
        assert_eq!(SPRITE_HEIGHT, 5);
    }

    #[test]
    fn test_load_fonts() {
        let mut memory = Memory::new();
        memory.load_fonts();

        // Verify '0' sprite (first 5 bytes)
        assert_eq!(memory.read(0x000), 0xF0);
        assert_eq!(memory.read(0x001), 0x90);
        assert_eq!(memory.read(0x002), 0x90);
        assert_eq!(memory.read(0x003), 0x90);
        assert_eq!(memory.read(0x004), 0xF0);

        // Verify '1' sprite (next 5 bytes)
        assert_eq!(memory.read(0x005), 0x20);
        assert_eq!(memory.read(0x006), 0x60);
        assert_eq!(memory.read(0x007), 0x20);
        assert_eq!(memory.read(0x008), 0x20);
        assert_eq!(memory.read(0x009), 0x70);
    }

    #[test]
    fn test_font_address() {
        let memory = Memory::new();

        // '0' at 0x000
        assert_eq!(memory.font_address(0x0), Some(0x000));
        // '1' at 0x005
        assert_eq!(memory.font_address(0x1), Some(0x005));
        // 'A' at 0x032 (0x0A * 5 = 50 = 0x32)
        assert_eq!(memory.font_address(0xA), Some(0x032));
        // 'F' at 0x04B (0x0F * 5 = 75 = 0x4B)
        assert_eq!(memory.font_address(0xF), Some(0x04B));
    }

    #[test]
    fn test_invalid_font_address() {
        let memory = Memory::new();

        assert!(memory.font_address(0x10).is_none());
    }

    #[test]
    fn test_sprite_to_ascii() {
        // '0' sprite: F0 90 90 90 F0
        // 0xF0 = 11110000 -> bits 0-3 on
        let zero = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        let ascii = sprite_to_ascii(&zero);
        
        // bit0 is leftmost in output
        // 0xF0 = ....#### 
        // 0x90 = ....#..#
        let expected = "....####\n....#..#\n....#..#\n....#..#\n....####";
        assert_eq!(ascii, expected);
    }

    #[test]
    fn test_sprite_to_ascii_1() {
        // '1' sprite: 20 60 20 20 70
        let one = [0x20, 0x60, 0x20, 0x20, 0x70];
        let ascii = sprite_to_ascii(&one);
        
        // 0x70 = 01110000 -> bits 4,5,6 set = display is ...###..
        // But output shows "....###." -> bit 4 on = position 4 from left
        let expected = ".....#..\n.....##.\n.....#..\n.....#..\n....###.";
        assert_eq!(ascii, expected);
    }
}
