//! Memory module for CHIP-8 emulator.
//!
//! CHIP-8 has 4KB (4096 bytes) of RAM.
//! - 0x000 to 0x1FF: Reserved for interpreter (typically used for built-in fonts)
//! - 0x200 to 0xFFF: Program memory (ROM loaded here)

/// CHIP-8 memory size: 4KB (4096 bytes)
pub const RAM_SIZE: usize = 4096;

/// Address where program ROM is loaded (and execution starts)
pub const ROM_LOAD_ADDRESS: u16 = 0x200;

/// The CHIP-8 interpreter reserved area (not used by ROMs)
pub const INTERPRETER_SIZE: usize = 0x200; // 512 bytes

pub const FONT_START: u16 = 0x000;


/// CHIP-8 memory structure
#[derive(Debug)]
pub struct Memory {
    /// Main RAM array (4KB)
    data: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; RAM_SIZE],
        }
    }

    /// Read a byte from memory at the given address.
    pub fn read(&self, addr: u16) -> u8 {
        assert!(
            addr < RAM_SIZE as u16,
            "Memory read out of bounds: {:#06x}",
            addr
        );
        self.data[addr as usize]
    }

    /// Read a 16-bit word from memory (big-endian: high byte first).
    pub fn read_word(&self, addr: u16) -> u16 {
        let high = self.read(addr) as u16;
        let low = self.read(addr + 1) as u16;
        (high << 8) | low
    }

    /// Write a byte to memory at the given address.
    pub fn write(&mut self, addr: u16, value: u8) {
        assert!(
            addr < RAM_SIZE as u16,
            "Memory write out of bounds: {:#06x}",
            addr
        );
        self.data[addr as usize] = value;
    }

    /// Write a 16-bit word to memory (big-endian: high byte first).
    pub fn write_word(&mut self, addr: u16, value: u16) {
        // 0x12AB as u8 -> AB
        self.write(addr, (value >> 8) as u8);
        self.write(addr + 1, value as u8);
    }

    /// Write a slice of bytes to memory starting at addr.
    pub fn write_slice(&mut self, addr: u16, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.write(addr + i as u16, byte);
        }
    }

    /// Load a ROM into memory starting at 0x200.
    /// Returns the size of the ROM that was loaded.
    pub fn load_rom(&mut self, rom: &[u8]) -> usize {
        let size = rom.len();
        assert!(
            ROM_LOAD_ADDRESS as usize + size <= RAM_SIZE,
            "ROM too large: {} bytes (max {})",
            size,
            RAM_SIZE - ROM_LOAD_ADDRESS as usize
        );

        for (i, &byte) in rom.iter().enumerate() {
            self.data[ROM_LOAD_ADDRESS as usize + i] = byte;
        }

        size
    }

    /// Get a slice of memory for reading (useful for debugging).
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Clear all memory to zero.
    pub fn reset(&mut self) {
        self.data = [0; RAM_SIZE];
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_memory_is_zeroed() {
        let memory = Memory::new();
        for byte in memory.data.iter() {
            assert_eq!(*byte, 0);
        }
    }

    #[test]
    fn test_read_write() {
        let mut memory = Memory::new();
        memory.write(0x123, 0xAB);
        assert_eq!(memory.read(0x123), 0xAB);
    }

    #[test]
    fn test_read_write_word() {
        let mut memory = Memory::new();
        memory.write_word(0x100, 0x1234);
        assert_eq!(memory.read_word(0x100), 0x1234);
    }

    #[test]
    fn test_load_rom() {
        let mut memory = Memory::new();
        let rom = vec![0x00, 0xE0, 0x12, 0x34];
        let size = memory.load_rom(&rom);

        assert_eq!(size, 4);
        assert_eq!(memory.read(ROM_LOAD_ADDRESS), 0x00);
        assert_eq!(memory.read(ROM_LOAD_ADDRESS + 1), 0xE0);
        assert_eq!(memory.read(ROM_LOAD_ADDRESS + 2), 0x12);
        assert_eq!(memory.read(ROM_LOAD_ADDRESS + 3), 0x34);
    }

    #[test]
    #[should_panic]
    fn test_read_out_of_bounds() {
        let memory = Memory::new();
        memory.read(RAM_SIZE as u16);
    }

    #[test]
    #[should_panic]
    fn test_write_out_of_bounds() {
        let mut memory = Memory::new();
        memory.write(RAM_SIZE as u16, 0xFF);
    }

    #[test]
    #[should_panic]
    fn test_rom_too_large() {
        let mut memory = Memory::new();
        let big_rom = vec![0u8; RAM_SIZE];
        memory.load_rom(&big_rom);
    }
}
