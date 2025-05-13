# CHIP-8 Emulator
![chip-8](https://github.com/user-attachments/assets/043c5885-6e61-44fa-97e8-df83bc8dfb1b)

CHIP-8 is a simple, interpreted, virtual machine that was initially used on some do-it-yourself computer systems in the 1970s and gained popularity through its use on the HP 48 series of calculators. Because it's relatively easy to implement, it's a popular target for emulator development.

## How it Works

This emulator aims to faithfully reproduce the behavior of a standard CHIP-8 interpreter. It works by:

1.  **Initialization:** Setting up the virtual machine's memory, registers, stack, and program counter.
2.  **Loading ROM:** Reading the CHIP-8 program (ROM) from a file into the emulator's memory, starting at address `0x200`.
3.  **Emulation Loop:** Continuously fetching, decoding, and executing CHIP-8 instructions.
    * **Fetch:** Reading the next two bytes (an opcode) from memory at the address pointed to by the program counter.
    * **Decode:** Interpreting the opcode to determine the instruction to be executed.
    * **Execute:** Performing the action specified by the instruction, which might involve modifying registers, memory, the display, or program flow.

### Supported Instructions

This emulator currently supports the following standard CHIP-8 instructions:

| Opcode | Mnemonic | Description                                                                 |
| :----- | :------- | :-------------------------------------------------------------------------- |
| `00E0` | `CLS`    | Clears the display.                                                       |
| `00EE` | `RET`    | Returns from a subroutine.                                                  |
| `1nnn` | `JP addr`| Jumps to address `nnn`.                                                     |
| `2nnn` | `CALL addr`| Calls subroutine at address `nnn`.                                        |
| `3xkk` | `SE Vx, byte`| Skips the next instruction if `Vx == kk`.                               |
| `4xkk` | `SNE Vx, byte`| Skips the next instruction if `Vx != kk`.                               |
| `5xy0` | `SE Vx, Vy`| Skips the next instruction if `Vx == Vy`.                               |
| `6xkk` | `LD Vx, byte`| Sets `Vx = kk`.                                                          |
| `7xkk` | `ADD Vx, byte`| Sets `Vx = Vx + kk`.                                                     |
| `8xy0` | `LD Vx, Vy`| Sets `Vx = Vy`.                                                          |
| `8xy1` | `OR Vx, Vy`| Sets `Vx = Vx OR Vy`.                                                     |
| `8xy2` | `AND Vx, Vy`| Sets `Vx = Vx AND Vy`.                                                    |
| `8xy3` | `XOR Vx, Vy`| Sets `Vx = Vx XOR Vy`.                                                    |
| `8xy4` | `ADD Vx, Vy`| Sets `Vx = Vx + Vy`, sets `VF = carry`.                                  |
| `8xy5` | `SUB Vx, Vy`| Sets `Vx = Vx - Vy`, sets `VF = NOT borrow`.                             |
| `8xy6` | `SHR Vx {, Vy}`| Sets `Vx = Vx SHR 1`. `VF` is set to the value of the least significant bit of `Vx` before the shift. |
| `8xy7` | `SUBN Vx, Vy`| Sets `Vx = Vy - Vx`, sets `VF = NOT borrow`.                             |
| `8xyE` | `SHL Vx {, Vy}`| Sets `Vx = Vx SHL 1`. `VF` is set to the value of the most significant bit of `Vx` before the shift. |
| `9xy0` | `SNE Vx, Vy`| Skips the next instruction if `Vx != Vy`.                               |
| `Annn` | `LD I, addr`| Sets `I = nnn`.                                                          |
| `Bnnn` | `JP V0, addr`| Jumps to address `nnn + V0`.                                            |
| `Cxkk` | `RND Vx, byte`| Sets `Vx = random byte AND kk`.                                         |
| `Dxyn` | `DRW Vx, Vy, nibble`| Draws a sprite at coordinate (`Vx`, `Vy`) that is `n` bytes tall using data starting at memory location `I`. `VF` is set to `1` if any set pixels are flipped to unset, and `0` otherwise. |
| `Ex9E` | `SKP Vx`   | Skips the next instruction if the key with the value of `Vx` is pressed. |
| `ExA1` | `SKNP Vx`  | Skips the next instruction if the key with the value of `Vx` is not pressed. |
| `Fx07` | `LD Vx, DT`| Sets `Vx = delay timer value`.                                           |
| `Fx0A` | `LD Vx, K` | Waits for a key press, then stores the value of the key in `Vx`.          |
| `Fx15` | `LD DT, Vx`| Sets the delay timer to the value of `Vx`.                               |
| `Fx18` | `LD ST, Vx`| Sets the sound timer to the value of `Vx`.                               |
| `Fx1E` | `ADD I, Vx`| Sets `I = I + Vx`.                                                        |
| `Fx29` | `LD F, Vx` | Sets `I` to the location of the sprite for the digit in `Vx`.             |
| `Fx33` | `LD B, Vx` | Stores the binary-coded decimal representation of `Vx` in memory locations `I`, `I+1`, and `I+2`. |
| `Fx55` | `LD [I], Vx`| Stores registers `V0` through `Vx` in memory starting at location `I`.   |
| `Fx65` | `LD Vx, [I]`| Reads registers `V0` through `Vx` from memory starting at location `I`.   |

## Requirements
Before you can build and run this emulator, you will need to have the following tools installed:
* **Rust and Cargo:** The Rust toolchain is necessary to build the backend. You can install it from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* **Node.js and pnpm:** These are required for building and running the frontend. You can install Node.js from [https://nodejs.org/](https://nodejs.org/) and then install pnpm globally using `npm install -g pnpm`.
* **wasm-pack:** This tool is used to build the Rust frontend code to WebAssembly. You can install it by following the instructions on its repository: [https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)

## How to build
To build and run the emulator, you will first build the frontend using `pnpm` and then run the Rust backend separately.
1.  **Navigate to the `frontend` directory:**
    ```bash
    cd frontend
    ```
2.  **Build the backend engine:**
    ```bash
    pnpm run build-engine
    ```
3.  **Start the frontend development server:**
    ```bash
    pnpm run dev
    ```

## Future Features (S-CHIP)

In the future, I plan to add support for [S-CHIP](http://devernay.free.fr/hacks/chip8/schip.txt). This would include:
* Larger screen size (128x64 pixels).
* Additional opcodes and functionalities.
* Potentially higher clock speeds.

