use std::fs;

struct Chip8 {
    memory: [u8; 4096],        // 4KB memory
    registers: [u8; 16],       // 16 8-bit registers (V0-VF)
    program_counter: u16,      // 16-bit program counter
    index_register: u16,       // 16-bit index register
    stack: Vec<u16>,           // Call stack
    delay_timer: u8,           // Delay timer
    sound_timer: u8,           // Sound timer
    display: [[bool; 64]; 32], // 64x32 display
    keyboard: [bool; 16],      // 16 key keyboard
}

impl Chip8 {
    fn new() -> Self {
        println!("[INIT] Creating new CHIP-8 emulator instance");
        Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            program_counter: 0x200, // Programs start at 0x200
            index_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; 64]; 32],
            keyboard: [false; 16],
        }
    }

    fn load_rom(&mut self, path: &str) {
        println!("[ROM] Loading ROM from: {}", path);
        match fs::read(path) {
            Ok(data) => {
                println!("[ROM] ROM loaded, size: {} bytes", data.len());
                println!("[ROM] TODO: Copy ROM data into memory starting at 0x200");
                // TODO: Copy data into memory[0x200..]
            }
            Err(e) => println!("[ERROR] Failed to load ROM: {}", e),
        }
    }

    fn fetch(&self) -> u16 {
        println!(
            "[FETCH] Fetching opcode at PC: 0x{:X}",
            self.program_counter
        );
        println!("[FETCH] TODO: Combine two bytes from memory into 16-bit opcode");
        // TODO: Combine memory[PC] and memory[PC+1]
        0x0000
    }

    fn decode_execute(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        println!(
            "[DECODE] Opcode: 0x{:04X} | nnn: 0x{:X}, nn: 0x{:02X}, n: 0x{:X}, x: {}, y: {}",
            opcode, nnn, nn, n, x, y
        );

        match opcode & 0xF000 {
            0x0000 => self.handle_0nnn(opcode),
            0x1000 => self.handle_1nnn(nnn),
            0x2000 => self.handle_2nnn(nnn),
            0x3000 => self.handle_3xnn(x, nn),
            0x4000 => self.handle_4xnn(x, nn),
            0x5000 => self.handle_5xy0(x, y),
            0x6000 => self.handle_6xnn(x, nn),
            0x7000 => self.handle_7xnn(x, nn),
            0x8000 => self.handle_8xy_(opcode, x, y),
            0x9000 => self.handle_9xy0(x, y),
            0xA000 => self.handle_Annn(nnn),
            0xB000 => self.handle_Bnnn(nnn),
            0xC000 => self.handle_Cxnn(x, nn),
            0xD000 => self.handle_Dxyn(x, y, n),
            0xE000 => self.handle_Ex__(opcode, x),
            0xF000 => self.handle_Fx__(opcode, x),
            _ => println!("[UNKNOWN] Unknown opcode: 0x{:04X}", opcode),
        }
    }

    fn handle_0nnn(&mut self, opcode: u16) {
        println!("[0x0nnn] TODO: Clear display or return from subroutine (based on opcode)");
    }

    fn handle_1nnn(&mut self, nnn: u16) {
        println!("[0x1nnn] Jump to address 0x{:X}", nnn);
        println!("[0x1nnn] TODO: Set PC = nnn");
    }

    fn handle_2nnn(&mut self, nnn: u16) {
        println!("[0x2nnn] Call subroutine at 0x{:X}", nnn);
        println!("[0x2nnn] TODO: Push current PC onto stack, set PC = nnn");
    }

    fn handle_3xnn(&mut self, x: usize, nn: u8) {
        println!(
            "[0x3xnn] Skip next instruction if VX (0x{:02X}) == 0x{:02X}",
            self.registers[x], nn
        );
        println!("[0x3xnn] TODO: Compare and increment PC if equal");
    }

    fn handle_4xnn(&mut self, x: usize, nn: u8) {
        println!(
            "[0x4xnn] Skip next instruction if VX (0x{:02X}) != 0x{:02X}",
            self.registers[x], nn
        );
        println!("[0x4xnn] TODO: Compare and increment PC if not equal");
    }

    fn handle_5xy0(&mut self, x: usize, y: usize) {
        println!(
            "[0x5xy0] Skip next instruction if VX ({}) == VY ({})",
            self.registers[x], self.registers[y]
        );
        println!("[0x5xy0] TODO: Compare registers and increment PC if equal");
    }

    fn handle_6xnn(&mut self, x: usize, nn: u8) {
        println!("[0x6xnn] Set VX to 0x{:02X}", nn);
        println!("[0x6xnn] TODO: Set V[{}] = 0x{:02X}", x, nn);
    }

    fn handle_7xnn(&mut self, x: usize, nn: u8) {
        println!(
            "[0x7xnn] Add 0x{:02X} to VX (currently 0x{:02X})",
            nn, self.registers[x]
        );
        println!("[0x7xnn] TODO: Add nn to V[{}] (handle overflow)", x);
    }

    fn handle_8xy_(&mut self, opcode: u16, x: usize, y: usize) {
        let n = opcode & 0x000F;
        match n {
            0x0 => {
                println!("[0x8xy0] Set VX to VY");
                println!("[0x8xy0] TODO: V[{}] = V[{}]", x, y);
            }
            0x1 => {
                println!("[0x8xy1] Set VX to VX | VY (bitwise OR)");
                println!("[0x8xy1] TODO: V[{}] = V[{}] | V[{}]", x, x, y);
            }
            0x2 => {
                println!("[0x8xy2] Set VX to VX & VY (bitwise AND)");
                println!("[0x8xy2] TODO: V[{}] = V[{}] & V[{}]", x, x, y);
            }
            0x3 => {
                println!("[0x8xy3] Set VX to VX ^ VY (bitwise XOR)");
                println!("[0x8xy3] TODO: V[{}] = V[{}] ^ V[{}]", x, x, y);
            }
            0x4 => {
                println!("[0x8xy4] Add VY to VX, set VF to carry");
                println!(
                    "[0x8xy4] TODO: Add V[{}] + V[{}], set VF based on overflow",
                    x, y
                );
            }
            0x5 => {
                println!("[0x8xy5] Subtract VY from VX, set VF to borrow");
                println!(
                    "[0x8xy5] TODO: V[{}] = V[{}] - V[{}], set VF based on borrow",
                    x, x, y
                );
            }
            0x6 => {
                println!("[0x8xy6] Shift VX right by 1");
                println!(
                    "[0x8xy6] TODO: Shift V[{}] right, set VF to LSB before shift",
                    x
                );
            }
            0x7 => {
                println!("[0x8xy7] Set VX to VY - VX, set VF to borrow");
                println!(
                    "[0x8xy7] TODO: V[{}] = V[{}] - V[{}], set VF based on borrow",
                    x, y, x
                );
            }
            0xE => {
                println!("[0x8xyE] Shift VX left by 1");
                println!(
                    "[0x8xyE] TODO: Shift V[{}] left, set VF to MSB before shift",
                    x
                );
            }
            _ => println!("[0x8xy_] Unknown sub-opcode: 0x{:04X}", opcode),
        }
    }

    fn handle_9xy0(&mut self, x: usize, y: usize) {
        println!(
            "[0x9xy0] Skip next instruction if VX ({}) != VY ({})",
            self.registers[x], self.registers[y]
        );
        println!("[0x9xy0] TODO: Compare registers and increment PC if not equal");
    }

    fn handle_Annn(&mut self, nnn: u16) {
        println!("[0xAnnn] Set index register to 0x{:X}", nnn);
        println!("[0xAnnn] TODO: Set I = 0x{:X}", nnn);
    }

    fn handle_Bnnn(&mut self, nnn: u16) {
        println!("[0xBnnn] Jump to address 0x{:X} + V0", nnn);
        println!(
            "[0xBnnn] TODO: Set PC = 0x{:X} + V0 (0x{:02X})",
            nnn, self.registers[0]
        );
    }

    fn handle_Cxnn(&mut self, x: usize, nn: u8) {
        println!("[0xCxnn] Set VX to random number AND 0x{:02X}", nn);
        println!(
            "[0xCxnn] TODO: Generate random byte and AND with 0x{:02X}",
            nn
        );
    }

    fn handle_Dxyn(&mut self, x: usize, y: usize, n: u8) {
        println!(
            "[0xDxyn] Draw sprite at VX ({}), VY ({}) with height {}",
            self.registers[x], self.registers[y], n
        );
        println!(
            "[0xDxyn] TODO: Draw n-byte sprite from memory[I] at (VX, VY), set VF to collision"
        );
    }

    fn handle_Ex__(&mut self, opcode: u16, x: usize) {
        let nn = opcode & 0x00FF;
        match nn {
            0x9E => {
                println!("[0xEx9E] Skip next instruction if key VX is pressed");
                println!(
                    "[0xEx9E] TODO: Check if key[V[{}]] is pressed, increment PC if true",
                    x
                );
            }
            0xA1 => {
                println!("[0xExA1] Skip next instruction if key VX is NOT pressed");
                println!(
                    "[0xExA1] TODO: Check if key[V[{}]] is not pressed, increment PC if true",
                    x
                );
            }
            _ => println!("[0xEx__] Unknown sub-opcode: 0x{:04X}", opcode),
        }
    }

    fn handle_Fx__(&mut self, opcode: u16, x: usize) {
        let nn = opcode & 0x00FF;
        match nn {
            0x07 => {
                println!("[0xFx07] Set VX to delay timer value");
                println!(
                    "[0xFx07] TODO: V[{}] = delay_timer ({})",
                    x, self.delay_timer
                );
            }
            0x0A => {
                println!("[0xFx0A] Wait for key press, store in VX");
                println!(
                    "[0xFx0A] TODO: Pause execution until key pressed, then V[{}] = key",
                    x
                );
            }
            0x15 => {
                println!("[0xFx15] Set delay timer to VX");
                println!(
                    "[0xFx15] TODO: delay_timer = V[{}] (0x{:02X})",
                    x, self.registers[x]
                );
            }
            0x18 => {
                println!("[0xFx18] Set sound timer to VX");
                println!(
                    "[0xFx18] TODO: sound_timer = V[{}] (0x{:02X})",
                    x, self.registers[x]
                );
            }
            0x1E => {
                println!("[0xFx1E] Add VX to index register");
                println!(
                    "[0xFx1E] TODO: I += V[{}] (0x{:02X}), handle overflow",
                    x, self.registers[x]
                );
            }
            0x29 => {
                println!("[0xFx29] Set index to font character VX");
                println!(
                    "[0xFx29] TODO: I = memory address of font digit V[{}] (0x{:02X})",
                    x, self.registers[x]
                );
            }
            0x33 => {
                println!("[0xFx33] Store BCD of VX at I, I+1, I+2");
                println!("[0xFx33] TODO: Store binary-coded decimal of V[{}] at I", x);
            }
            0x55 => {
                println!("[0xFx55] Store registers V0 to VX at I");
                println!("[0xFx55] TODO: Store V0..V[{}] starting at memory[I]", x);
            }
            0x65 => {
                println!("[0xFx65] Load registers V0 to VX from I");
                println!("[0xFx65] TODO: Load V0..V[{}] from memory[I]", x);
            }
            _ => println!("[0xFx__] Unknown sub-opcode: 0x{:04X}", opcode),
        }
    }

    fn update_timers(&mut self) {
        println!("[TIMERS] Updating timers...");
        println!("[TIMERS] TODO: Decrement delay_timer and sound_timer if > 0");
        if self.delay_timer > 0 {
            println!(
                "[TIMERS]   delay_timer: {} -> {}",
                self.delay_timer,
                self.delay_timer - 1
            );
        }
        if self.sound_timer > 0 {
            println!(
                "[TIMERS]   sound_timer: {} -> {}",
                self.sound_timer,
                self.sound_timer - 1
            );
            println!("[TIMERS]   TODO: Beep if sound_timer > 0");
        }
    }

    fn run_cycle(&mut self) {
        println!("\n[CYCLE] ===== FETCH-DECODE-EXECUTE CYCLE =====");
        let opcode = self.fetch();
        self.decode_execute(opcode);
        println!("[CYCLE] TODO: Increment PC to next instruction (PC += 2)");
        self.update_timers();
        println!("[CYCLE] TODO: Render display to screen");
    }
}

fn main() {
    println!("╔═════════════════════════════╗");
    println!("║       CHIP-8 Emulator       ║");
    println!("╚═════════════════════════════╝\n");

    let mut chip8 = Chip8::new();

    println!("\n[MAIN] TODO: Load ROM from command-line argument or file");
    // chip8.load_rom("path/to/rom.ch8");

    println!("\n[MAIN] Starting emulation loop...");
    println!("[MAIN] TODO: Implement timing (60 Hz main cycle, 500+ Hz for timers)");
    println!("[MAIN] TODO: Handle keyboard input events");
    println!("[MAIN] TODO: Render display buffer to screen\n");

    // Simulate a few cycles with a dummy opcode
    for i in 1..=3 {
        println!("\n--- Cycle {} ---", i);
        chip8.run_cycle();
    }

    println!("\n[MAIN] TODO: Implement proper event loop");
    println!("[MAIN] TODO: Implement display rendering with external graphics library (SDL2/wgpu)");
    println!("[MAIN] TODO: Implement keyboard input handling");
    println!("[MAIN] Emulation finished!");
}
