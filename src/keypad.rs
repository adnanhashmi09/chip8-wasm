//! Keypad module for CHIP-8 emulator.
//!
//! Chip-8 has a 16-key hexadecimal keypad.
//! Keys are typically mapped to a modern keyboard.

/// Number of keys on the Chip-8 keypad
pub const KEY_COUNT: usize = 16;

/// Chip-8 key mapping to modern keyboard (most common mapping).
/// Key 0x0-0xF maps to these ASCII chars.
pub const KEY_MAP: [(u8, char); 16] = [
    (0x1, '1'),  // Key 0x1
    (0x2, '2'),  // Key 0x2
    (0x3, '3'),  // Key 0x3
    (0xC, '4'),  // Key 0xC
    (0x4, 'q'),  // Key 0x4
    (0x5, 'w'),  // Key 0x5
    (0x6, 'e'),  // Key 0x6
    (0xD, 'r'),  // Key 0xD
    (0x7, 'a'),  // Key 0x7
    (0x8, 's'),  // Key 0x8
    (0x9, 'd'),  // Key 0x9
    (0xE, 'f'),  // Key 0xE
    (0xA, 'z'),  // Key 0xA
    (0x0, 'x'),  // Key 0x0
    (0xB, 'c'),  // Key 0xB
    (0xF, 'v'),  // Key 0xF
];

/// The Chip-8 keypad structure.
#[derive(Debug, Default)]
pub struct Keypad {
    /// State of all 16 keys (true = pressed)
    keys: [bool; KEY_COUNT],
}

impl Keypad {
    /// Create a new keypad with all keys released.
    pub fn new() -> Self {
        Keypad {
            keys: [false; KEY_COUNT],
        }
    }

    /// Press a key (set to pressed state).
    ///
    /// # Panics
    /// Panics if key is not in range 0-15.
    pub fn press(&mut self, key: u8) {
        assert!(key < KEY_COUNT as u8, "Invalid key: {}", key);
        self.keys[key as usize] = true;
    }

    /// Release a key (set to released state).
    ///
    /// # Panics
    /// Panics if key is not in range 0-15.
    pub fn release(&mut self, key: u8) {
        assert!(key < KEY_COUNT as u8, "Invalid key: {}", key);
        self.keys[key as usize] = false;
    }

    /// Check if a key is pressed.
    ///
    /// # Panics
    /// Panics if key is not in range 0-15.
    pub fn is_pressed(&self, key: u8) -> bool {
        assert!(key < KEY_COUNT as u8, "Invalid key: {}", key);
        self.keys[key as usize]
    }

    /// Get the state of all keys as a slice.
    pub fn keys(&self) -> &[bool; KEY_COUNT] {
        &self.keys
    }

    /// Reset all keys to released.
    pub fn reset(&mut self) {
        self.keys = [false; KEY_COUNT];
    }

    /// Wait for a key press and return the key.
    ///
    /// In a real implementation, this would block until a key is pressed.
    /// For the WASM implementation, we'll handle this differently.
    pub fn wait_for_key(&mut self) -> Option<u8> {
        for (i, &pressed) in self.keys.iter().enumerate() {
            if pressed {
                return Some(i as u8);
            }
        }
        None
    }

    /// Set key state from keyboard character.
    ///
    /// Maps the keyboard character to the corresponding Chip-8 key.
    pub fn set_from_keycode(&mut self, keycode: char, pressed: bool) {
        for (chip_key, k) in KEY_MAP.iter() {
            if *k == keycode {
                self.keys[*chip_key as usize] = pressed;
                return;
            }
        }
    }

    /// Get the Chip-8 key for a keyboard character, if valid.
    pub fn keycode_to_chip_key(keycode: char) -> Option<u8> {
        for (chip_key, k) in KEY_MAP.iter() {
            if *k == keycode {
                return Some(*chip_key);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_keypad_all_released() {
        let keypad = Keypad::new();
        for i in 0..KEY_COUNT {
            assert!(!keypad.is_pressed(i as u8));
        }
    }

    #[test]
    fn test_press_release() {
        let mut keypad = Keypad::new();
        
        keypad.press(0x5);
        assert!(keypad.is_pressed(0x5));
        assert!(!keypad.is_pressed(0x4));
        
        keypad.release(0x5);
        assert!(!keypad.is_pressed(0x5));
    }

    #[test]
    fn test_reset() {
        let mut keypad = Keypad::new();
        keypad.press(0x1);
        keypad.press(0x2);
        keypad.press(0x3);
        
        keypad.reset();
        assert!(!keypad.is_pressed(0x1));
        assert!(!keypad.is_pressed(0x2));
        assert!(!keypad.is_pressed(0x3));
    }

    #[test]
    fn test_wait_for_key() {
        let mut keypad = Keypad::new();
        assert_eq!(keypad.wait_for_key(), None);
        
        keypad.press(0xA);
        assert_eq!(keypad.wait_for_key(), Some(0xA));
    }

    #[test]
    fn test_set_from_keycode() {
        let mut keypad = Keypad::new();
        
        // '1' maps to key 0x1
        keypad.set_from_keycode('1', true);
        assert!(keypad.is_pressed(0x1));
        
        keypad.set_from_keycode('1', false);
        assert!(!keypad.is_pressed(0x1));
    }

    #[test]
    fn test_keycode_to_chip_key() {
        assert_eq!(Keypad::keycode_to_chip_key('1'), Some(0x1));
        assert_eq!(Keypad::keycode_to_chip_key('x'), Some(0x0));
        assert_eq!(Keypad::keycode_to_chip_key('?'), None);
    }

    #[test]
    #[should_panic]
    fn test_invalid_key_press() {
        let mut keypad = Keypad::new();
        keypad.press(0xFF);
    }

    #[test]
    #[should_panic]
    fn test_invalid_key_release() {
        let mut keypad = Keypad::new();
        keypad.release(0x10);
    }
}
