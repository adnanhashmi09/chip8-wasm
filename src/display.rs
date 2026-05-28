//! Display module for CHIP-8 emulator.
//!
//! CHIP-8 has a 64x32 pixel display.
//! Each pixel is 1 bit (on/off), stored as a byte.
//! Total: 64 × 32 = 2048 bytes.

use std::fs;

/// Display width in pixels
pub const DISPLAY_WIDTH: usize = 64;

/// Display height in pixels
pub const DISPLAY_HEIGHT: usize = 32;

/// Total number of pixels
pub const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

/// Row 1 height for standard sprite drawing
pub const SPRITE_ROW_HEIGHT: usize = 5;

/// The CHIP-8 display structure
#[derive(Debug)]
pub struct Display {
    /// Framebuffer: 2048 bytes, each byte is 0 (off) or 1 (on)
    /// Pixel at (x, y) is at index y * DISPLAY_WIDTH + x
    pixels: [u8; DISPLAY_SIZE],
}

impl Display {
    /// Create a new, cleared display
    pub fn new() -> Self {
        Display {
            pixels: [0; DISPLAY_SIZE],
        }
    }

    /// Clear the entire display (set all pixels to off)
    pub fn clear(&mut self) {
        self.pixels = [0; DISPLAY_SIZE];
    }

    /// Get the pixel state at (x, y).
    ///
    /// Coordinates are NOT wrapped - out of bounds returns 0.
    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return 0;
        }
        self.pixels[y * DISPLAY_WIDTH + x]
    }

    /// Set the pixel at (x, y) to on (value = 1).
    ///
    /// Coordinates are NOT wrapped - out of bounds does nothing.
    pub fn set_pixel(&mut self, x: usize, y: usize) {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return;
        }
        self.pixels[y * DISPLAY_WIDTH + x] = 1;
    }

    /// Clear the pixel at (x, y) (set to 0).
    ///
    /// Coordinates are NOT wrapped - out of bounds does nothing.
    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return;
        }
        self.pixels[y * DISPLAY_WIDTH + x] = 0;
    }

    /// Toggle the pixel at (x, y).
    ///
    /// Coordinates are NOT wrapped - out of bounds does nothing.
    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return;
        }
        let idx = y * DISPLAY_WIDTH + x;
        self.pixels[idx] ^= 1;
    }

    /// Draw a sprite at position (x, y) using XOR.
    ///
    /// Each byte in the sprite represents one row of 8 pixels (bit 7 = leftmost).
    /// Drawing wraps around the screen edges.
    ///
    /// # Arguments
    /// * `x` - X coordinate (top-left of sprite)
    /// * `y` - Y coordinate (top-left of sprite)
    /// * `sprite` - Slice of bytes representing sprite rows
    ///
    /// # Returns
    /// * `true` if any pixel was erased (collision - pixel was already on)
    /// * `false` if no collision occurred
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (row_idx, &sprite_row) in sprite.iter().enumerate() {
            let y_pos = (y + row_idx) % DISPLAY_HEIGHT;

            // Each byte = 8 pixels, iterate over bits
            for bit in 0..8 {
                let x_pos = (x + bit) % DISPLAY_WIDTH;
                // Bit 7 = leftmost pixel of hex byte, so sprite[bit7]→x+0
                let pixel = (sprite_row >> (7 - bit)) & 1;

                if pixel == 1 {
                    let idx = y_pos * DISPLAY_WIDTH + x_pos;

                    // XOR: if pixel was on, it's now off (collision!)
                    if self.pixels[idx] == 1 {
                        collision = true;
                    }

                    // Toggle the pixel
                    self.pixels[idx] ^= 1;
                }
            }
        }

        collision
    }

    /// Get a reference to the raw framebuffer for rendering.
    pub fn pixels(&self) -> &[u8; DISPLAY_SIZE] {
        &self.pixels
    }

    /// Get a mutable reference to the raw framebuffer.
    pub fn pixels_mut(&mut self) -> &mut [u8; DISPLAY_SIZE] {
        &mut self.pixels
    }

    /// Save the display to a PBM (portable bitmap) file for debugging.
    ///
    /// # Arguments
    /// * `filename` - Path to save the file
    #[cfg(test)]
    pub fn save_pbm(&self, filename: &str) -> std::io::Result<()> {
        use std::io::Write;

        let mut file = fs::File::create(filename)?;

        // PBM P1 format: text-based, easy to inspect
        writeln!(file, "P1")?;
        writeln!(file, "{} {}", DISPLAY_WIDTH, DISPLAY_HEIGHT)?;

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                write!(file, "{} ", self.get_pixel(x, y))?;
            }
            writeln!(file)?;
        }

        Ok(())
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel = self.get_pixel(x, y);
                write!(f, "{}", if pixel == 1 { '█' } else { '·' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_display_is_cleared() {
        let display = Display::new();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                assert_eq!(display.get_pixel(x, y), 0);
            }
        }
    }

    #[test]
    fn test_set_and_get_pixel() {
        let mut display = Display::new();
        display.set_pixel(10, 5);
        assert_eq!(display.get_pixel(10, 5), 1);
        assert_eq!(display.get_pixel(11, 5), 0);
    }

    #[test]
    fn test_clear_pixel() {
        let mut display = Display::new();
        display.set_pixel(5, 5);
        display.set_pixel(5, 5);
        assert_eq!(display.get_pixel(5, 5), 1);
        display.clear_pixel(5, 5);
        assert_eq!(display.get_pixel(5, 5), 0);
    }

    #[test]
    fn test_out_of_bounds_is_zero() {
        let display = Display::new();
        assert_eq!(display.get_pixel(100, 10), 0);
        assert_eq!(display.get_pixel(10, 100), 0);
        assert_eq!(display.get_pixel(64, 32), 0);
    }

    #[test]
    fn test_clear() {
        let mut display = Display::new();
        display.set_pixel(0, 0);
        display.set_pixel(63, 31);
        display.clear();
        assert_eq!(display.get_pixel(0, 0), 0);
        assert_eq!(display.get_pixel(63, 31), 0);
    }

    #[test]
    fn test_toggle() {
        let mut display = Display::new();
        assert_eq!(display.get_pixel(5, 5), 0);
        display.toggle_pixel(5, 5);
        assert_eq!(display.get_pixel(5, 5), 1);
        display.toggle_pixel(5, 5);
        assert_eq!(display.get_pixel(5, 5), 0);
    }

    #[test]
    fn test_draw_sprite_simple() {
        let mut display = Display::new();

        // Draw a single pixel sprite at (0, 0)
        let sprite = [0x80]; // 10000000 - one pixel on
        display.draw_sprite(0, 0, &sprite);

        assert_eq!(display.get_pixel(0, 0), 1);
        assert_eq!(display.get_pixel(7, 0), 0);
    }

    #[test]
    fn test_draw_sprite_full_byte() {
        let mut display = Display::new();

        // Draw a full byte (8 pixels horizontally)
        let sprite = [0xFF]; // 11111111 - all 8 pixels on
        display.draw_sprite(0, 0, &sprite);

        for x in 0..8 {
            assert_eq!(display.get_pixel(x, 0), 1);
        }
    }

    #[test]
    fn test_draw_sprite_vertical() {
        let mut display = Display::new();

        // Draw 3 bytes vertically (a vertical line)
        let sprite = [0x80, 0x80, 0x80];
        display.draw_sprite(5, 0, &sprite);

        assert_eq!(display.get_pixel(5, 0), 1);
        assert_eq!(display.get_pixel(5, 1), 1);
        assert_eq!(display.get_pixel(5, 2), 1);
        assert_eq!(display.get_pixel(5, 3), 0);
    }

    #[test]
    fn test_draw_sprite_no_collision() {
        let mut display = Display::new();

        let sprite = [0xFF, 0xFF];
        let collision = display.draw_sprite(0, 0, &sprite);

        assert!(!collision);
    }

    #[test]
    fn test_draw_sprite_collision() {
        let mut display = Display::new();

        let sprite = [0xFF];
        display.draw_sprite(0, 0, &sprite);

        // Drawing same sprite again should detect collision
        let collision = display.draw_sprite(0, 0, &sprite);

        assert!(collision);
    }

    #[test]
    fn test_draw_sprite_xor() {
        let mut display = Display::new();

        // Draw first time
        let sprite = [0xFF];
        display.draw_sprite(0, 0, &sprite);

        // All pixels should be on
        for x in 0..8 {
            assert_eq!(display.get_pixel(x, 0), 1);
        }

        // Draw second time (XOR should turn them off)
        display.draw_sprite(0, 0, &sprite);

        // All pixels should be off
        for x in 0..8 {
            assert_eq!(display.get_pixel(x, 0), 0);
        }
    }

    #[test]
    fn test_draw_sprite_wrap_x() {
        let mut display = Display::new();

        // Draw at x=61 (only 3 bits will fit before wrapping)
        let sprite = [0xFF]; // 8 pixels
        display.draw_sprite(61, 0, &sprite);

        // Pixels at 61, 62, 63, then wraps to 0, 1, 2, 3, 4
        assert_eq!(display.get_pixel(61, 0), 1);
        assert_eq!(display.get_pixel(62, 0), 1);
        assert_eq!(display.get_pixel(63, 0), 1);
        assert_eq!(display.get_pixel(0, 0), 1);
        assert_eq!(display.get_pixel(1, 0), 1);
        assert_eq!(display.get_pixel(2, 0), 1);
        assert_eq!(display.get_pixel(4, 0), 1);
    }

    #[test]
    fn test_draw_sprite_wrap_y() {
        let mut display = Display::new();

        // Draw at y=30 (only 2 rows will fit before wrapping)
        let sprite = [0xFF, 0xFF, 0xFF]; // 3 bytes = 3 rows
        display.draw_sprite(0, 30, &sprite);

        // Should be at rows 30, 31, then wraps to 0
        assert_eq!(display.get_pixel(0, 30), 1);
        assert_eq!(display.get_pixel(0, 31), 1);
        assert_eq!(display.get_pixel(0, 0), 1);
    }

    #[test]
    fn test_draw_zero_sprite() {
        let mut display = Display::new();

        // Drawing zeros shouldn't affect anything
        let sprite = [0x00, 0x00, 0x00];
        let collision = display.draw_sprite(0, 0, &sprite);

        assert!(!collision);
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                assert_eq!(display.get_pixel(x, y), 0);
            }
        }
    }

    #[test]
    fn test_draw_font_zero() {
        let mut display = Display::new();

        // Standard '0' sprite (5 rows)
        let zero_sprite = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        display.draw_sprite(5, 5, &zero_sprite);

        // Row 0: 0xF0 = 11110000, bits 0-3 are set (rightmost bits in byte)
        // So pixels 5,6,7,8 are on (leftmost on display since sprite is at x=5)
        assert_eq!(display.get_pixel(5, 5), 1);  // bit 0
        assert_eq!(display.get_pixel(6, 5), 1);  // bit 1
        assert_eq!(display.get_pixel(7, 5), 1);  // bit 2
        assert_eq!(display.get_pixel(8, 5), 1);  // bit 3
        assert_eq!(display.get_pixel(9, 5), 0);  // bit 4 (off)

        // Row 1: 0x90 = 10010000, bits 0 and 3 are set
        assert_eq!(display.get_pixel(5, 6), 1);  // bit 0 (left wall)
        assert_eq!(display.get_pixel(6, 6), 0);  // bit 1
        assert_eq!(display.get_pixel(7, 6), 0);  // bit 2
        assert_eq!(display.get_pixel(8, 6), 1);  // bit 3 (right wall)

        // Row 4 (bottom): 0xF0 = 11110000, same as top
        assert_eq!(display.get_pixel(5, 9), 1);  // bit 0
        assert_eq!(display.get_pixel(8, 9), 1);  // bit 3
    }

    #[test]
    fn test_display_to_string() {
        let mut display = Display::new();
        display.set_pixel(0, 0);
        display.set_pixel(1, 0);
        display.set_pixel(0, 1);

        let output = format!("{}", display);
        assert!(output.contains('█'));
    }
}
