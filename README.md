# Chip-8 WebAssembly Emulator

A CHIP-8 emulator written in Rust, compiled to WebAssembly for running in the browser.

## Architecture

- **Memory**: 4KB RAM (0x000-0xFFF)
- **ROM Load Address**: 0x200
- **Font Sprites**: 16 characters (0-9, A-F) at 0x000-0x04F, each 8x5 pixels
- **Compiled to WASM** for browser execution

## Modules

| Module | Description |
|--------|-------------|
| `memory` | 4KB RAM with read/write operations |
| `fonts` | Built-in 16-character sprite set |

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Project Structure

```
chip8/
├── src/
│   ├── lib.rs      # Crate root
│   ├── memory.rs   # RAM module
│   └── fonts.rs    # Sprite fonts
└── Cargo.toml
```
