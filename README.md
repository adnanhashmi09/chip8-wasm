# Chip-8 WebAssembly Emulator

A CHIP-8 emulator written in Rust, compiled to WebAssembly for running in the browser.

## Building

### Prerequisites
- Rust (with `wasm32-unknown-unknown` target)
- wasm-bindgen

### Install wasm-pack (if not installed)
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Or install via cargo:
```bash
cargo install wasm-pack
```

### Build WASM
```bash
wasm-pack build --target web
```

This generates:
- `pkg/chip8.js` - JavaScript bindings
- `pkg/chip8_bg.wasm` - WebAssembly binary

### Run the Demo
```bash
# Python (from the www directory)
python -m http.server 8080

# Or Node.js
npx serve .
```

Then open http://localhost:8080 in your browser.

## Architecture

```
www/
├── index.html      # Main page
└── index.js        # JS glue + game loop

Rust (src/):
├── lib.rs          # Crate exports
├── memory.rs       # 4KB RAM
├── fonts.rs        # 16 sprites (0-9, A-F)
├── display.rs      # 64x32 pixel buffer
├── cpu/
│   ├── mod.rs      # CPU + all opcodes
│   └── stack.rs    # 16-level stack
├── keypad.rs       # 16-key keyboard
├── timers.rs       # delay + sound timers
├── chip8.rs        # Complete emulator
└── wasm.rs          # WASM bindings for JS
```

## Controls

```
┌───┬───┬───┬───┐       Key    │ Key
│ 1 │ 2 │ 3 │ 4 │       ─────┼─────
├───┼───┼───┼───┤       1,2,3,4│ 1,2,3,C  │ Mapping to 0-9,A-F
│ Q │ W │ E │ R │  →    Q,W,E,R│ 4,5,6,D
├───┼───┼───┼───┤       A,S,D,F│ 7,8,9,E
│ A │ S │ D │ F │       Z,X,C,V│ A,0,B,F
├───┼───┼───┼───┤
│ Z │ X │ C │ V │
└───┴───┴───┴───┘
```

## Controls

Upload any `.ch8` Chip-8 ROM and play!
