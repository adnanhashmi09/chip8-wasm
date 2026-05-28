# Chip-8 WebAssembly Emulator

A CHIP-8 emulator written in Rust, compiled to WebAssembly for running in the browser.

## Architecture

- **Memory**: 4KB RAM
- **ROM Load Address**: 0x200
- **Compiled to WASM** for browser execution

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```
