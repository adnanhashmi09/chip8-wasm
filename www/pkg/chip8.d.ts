/* tslint:disable */
/* eslint-disable */

export class Chip8Wasm {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get display height.
     */
    static display_height(): number;
    /**
     * Get display width.
     */
    static display_width(): number;
    /**
     * Get delay timer value.
     */
    get_delay_timer(): number;
    /**
     * Get the display buffer (64x32 pixels as RGBA).
     */
    get_display(): Uint8Array;
    /**
     * Get index register.
     */
    get_index(): number;
    /**
     * Get program counter (for debugging).
     */
    get_pc(): number;
    /**
     * Get register value (0-15).
     */
    get_register(index: number): number;
    /**
     * Get sound timer value.
     */
    get_sound_timer(): number;
    /**
     * Check if a key is pressed.
     */
    is_key_pressed(key: number): boolean;
    /**
     * Press a key (0-15).
     */
    key_press(key: number): void;
    /**
     * Release a key (0-15).
     */
    key_release(key: number): void;
    /**
     * Load a demo program.
     */
    load_demo(): void;
    /**
     * Load a ROM from a byte array.
     */
    load_rom(data: Uint8Array): number;
    /**
     * Create a new Chip-8 emulator instance.
     */
    constructor();
    /**
     * Reset the emulator to initial state.
     */
    reset(): void;
    /**
     * Set key from keyboard character.
     */
    set_key(keycode: string, pressed: boolean): void;
    /**
     * Execute a single instruction.
     */
    step(): void;
    /**
     * Execute multiple instructions (one frame).
     */
    update(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_chip8wasm_free: (a: number, b: number) => void;
    readonly chip8wasm_new: () => number;
    readonly chip8wasm_reset: (a: number) => void;
    readonly chip8wasm_load_rom: (a: number, b: number, c: number) => [number, number, number];
    readonly chip8wasm_load_demo: (a: number) => void;
    readonly chip8wasm_step: (a: number) => void;
    readonly chip8wasm_update: (a: number) => void;
    readonly chip8wasm_get_display: (a: number) => [number, number];
    readonly chip8wasm_key_press: (a: number, b: number) => void;
    readonly chip8wasm_key_release: (a: number, b: number) => void;
    readonly chip8wasm_set_key: (a: number, b: number, c: number) => void;
    readonly chip8wasm_is_key_pressed: (a: number, b: number) => number;
    readonly chip8wasm_get_delay_timer: (a: number) => number;
    readonly chip8wasm_get_sound_timer: (a: number) => number;
    readonly chip8wasm_get_pc: (a: number) => number;
    readonly chip8wasm_get_register: (a: number, b: number) => number;
    readonly chip8wasm_get_index: (a: number) => number;
    readonly chip8wasm_display_width: () => number;
    readonly chip8wasm_display_height: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
