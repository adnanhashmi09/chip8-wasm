//! Minimal WASM Chip-8 emulator

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8Wasm {
    v: [u8; 16],
    i: u16,
    pc: u16,
    memory: [u8; 4096],
    display: [u8; 2048],
    stack: [u16; 16],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
}

#[wasm_bindgen]
impl Chip8Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8Wasm {
        let mut chip8 = Chip8Wasm {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            memory: [0; 4096],
            display: [0; 2048],
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        };
        chip8.load_fonts();
        chip8
    }

    fn load_fonts(&mut self) {
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for (i, &font_byte) in fonts.iter().enumerate() {
            self.memory[i] = font_byte;
        }
    }

    pub fn reset(&mut self) {
        self.v = [0; 16];
        self.i = 0;
        self.pc = 0x200;
        self.sp = 0;
        self.display = [0; 2048];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.load_fonts();
    }

    pub fn load_demo(&mut self) {
        let demo: [u8; 12] = [
            0x00, 0xE0, // CLS
            0x60, 0x00, // LD V0, 0
            0x61, 0x00, // LD V1, 0
            0xA0, 0x00, // LD I, 0
            0xD0, 0x15, // DRW V0, V1, 5
            0x12, 0x0A, // JP 0x20A
        ];
        for (i, &byte) in demo.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
        self.pc = 0x200;
    }

    pub fn step(&mut self) {
        if self.pc >= 4096 {
            return;
        }
        
        let opcode = ((self.memory[self.pc as usize] as u16) << 8) 
                  | (self.memory[self.pc as usize + 1] as u16);
        self.pc += 2;
        
        let op = (opcode >> 12) as u8;
        let vx = ((opcode >> 8) & 0xF) as usize;
        let vy = ((opcode >> 4) & 0xF) as usize;
        let nnn = opcode & 0xFFF;
        let nn = (opcode & 0xFF) as u8;
        let n = (opcode & 0xF) as u8;
        
        match op {
            0x0 => {
                if nnn == 0x0E0 {
                    self.display = [0; 2048];
                } else if nnn == 0x0EE {
                    if self.sp > 0 {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp];
                    }
                }
            }
            0x1 => {
                self.pc = nnn;
            }
            0x2 => {
                if self.sp < 16 {
                    self.stack[self.sp] = self.pc;
                    self.sp += 1;
                }
                self.pc = nnn;
            }
            0x6 => {
                self.v[vx] = nn;
            }
            0x7 => {
                self.v[vx] = self.v[vx].wrapping_add(nn);
            }
            0xA => {
                self.i = nnn;
            }
            0xD => {
                self.draw_sprite(vx, vy, n);
            }
            _ => {}
        }
    }

    fn draw_sprite(&mut self, vx: usize, vy: usize, height: u8) {
        let x = self.v[vx] as usize;
        let y = self.v[vy] as usize;
        
        self.v[0xF] = 0;
        
        for row in 0..height {
            let sprite_byte = self.memory[self.i as usize + row as usize];
            let y_pos = (y + row as usize) % 32;
            
            for col in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - col)) & 1;
                if sprite_pixel == 1 {
                    let x_pos = (x + col) % 64;
                    let idx = y_pos * 64 + x_pos;
                    
                    if self.display[idx] == 1 {
                        self.v[0xF] = 1;
                        self.display[idx] = 0;
                    } else {
                        self.display[idx] = 1;
                    }
                }
            }
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) -> Result<usize, JsValue> {
        if data.len() > 0xEFF {
            return Err(JsValue::from_str("ROM too large"));
        }
        for (i, &byte) in data.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
        self.pc = 0x200;
        Ok(data.len())
    }

    pub fn get_display(&self) -> Vec<u8> {
        self.display.to_vec()
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_index(&self) -> u16 {
        self.i
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn update(&mut self) {
        // Execute ~12 instructions per frame (60 FPS ≈ 720 IPS)
        for _ in 0..12 {
            self.step();
        }
        // Update timers (simplified - just decrement if > 0)
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
