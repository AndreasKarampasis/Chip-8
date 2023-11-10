use rand::random;

const RAM_SIZE: usize = 4096;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
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

pub struct Chip8 {
    pc: u16,
    ram: [u8; RAM_SIZE],
    display: [bool; DISPLAY_HEIGHT * DISPLAY_WIDTH], // a 64x32 monochrome display
    v: [u8; NUM_REGS], // general purpose registers, VF also doubles as flag register for overflow operations
    i: u16,            // used as a pointer for memory access
    sp: u16,           // stack pointer
    stack: [u16; STACK_SIZE], // used for calling and returning from subroutines
    keys: [bool; NUM_KEYS],
    dt: u8, // delay timer used for time-based game events
    st: u8, // sound timer used to trigger the audio beep
}

impl Chip8 {
    pub fn new() -> Self {
        let mut new_chip8 = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            display: [false; DISPLAY_HEIGHT * DISPLAY_WIDTH],
            v: [0; NUM_REGS],
            i: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };
        new_chip8.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_chip8
    }

    // Reset chip8 to a known state
    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.display = [false; DISPLAY_HEIGHT * DISPLAY_WIDTH];
        self.v = [0; NUM_REGS];
        self.i = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch
        let op: u16 = self.fetch();
        // Decode && Execute
        self.execute(op);
    }

    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start: usize = START_ADDR as usize;
        let end: usize = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    // Add given 16-bit value to the spot pointed by the stack pointer
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }
    // Return the value in the stack pointed by the stack pointer
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    /*
     * Read the instruction that PC is currently pointing at from memory.
     * An instruction is two bytes, so we need to read two successive bytes from memory
     * and combine them into one 16-bit instruction.
     */
    fn fetch(&mut self) -> u16 {
        let high_byte: u16 = self.ram[self.pc as usize] as u16;
        let low_byte: u16 = self.ram[(self.pc + 1) as usize] as u16;
        let op: u16 = (high_byte << 8) | low_byte;
        self.pc += 2;
        op
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // TODO: beep
            }
            self.st -= 1;
        }
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;
        match (digit1, digit2, digit3, digit4) {
            (0x0, 0x0, 0xE, 0x0) => {
                // Clear screen
                self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // Return from a subroutine
                let addr = self.pop();
                self.pc = addr;
            }
            (0x1, _, _, _) => {
                // Jump to location nnnn
                let nnn: u16 = op & 0x0FFF;
                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                // Call subroutine at nnn
                let nnn = op & 0x0FFF;
                self.push(self.pc);
                self.pc = nnn;
            }
            (0x3, _, _, _) => {
                // Skip the next instruction if Vx = nn;
                let x = digit2;
                let nn = (op & 0x00FF) as u8;
                if self.v[x as usize] == nn {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                // Skip the next instruction if Vx != nn
                let x = digit2;
                let nn = (op & 0x00FF) as u8;
                if self.v[x as usize] != nn {
                    self.pc += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                // Skip the next instruction if Vx = Vy
                let x = digit2;
                let y = digit3;
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => {
                // Set Vx = kk
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v[x] = nn;
            }
            (0x7, _, _, _) => {
                // Set Vx = Vx + kk
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            (0x8, _, _, 0x0) => {
                // Set Vx = Vy
                let x = digit2;
                let y = digit3;
                self.v[x as usize] = self.v[y as usize];
            }
            (0x8, _, _, 0x1) => {
                // Set Vx = Vy
                let x = digit2;
                let y = digit3;
                self.v[x as usize] |= self.v[y as usize];
            }
            (0x8, _, _, 0x2) => {
                // Set Vx = Vy
                let x = digit2;
                let y = digit3;
                self.v[x as usize] &= self.v[y as usize];
            }
            (0x8, _, _, 0x3) => {
                // Set Vx = Vy
                let x = digit2;
                let y = digit3;
                self.v[x as usize] ^= self.v[y as usize];
            }
            (0x8, _, _, 0x4) => {
                // Set Vx = Vx + Vy, set VF = carry.
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = new_vx;
                self.v[0xF] = if carry { 1 } else { 0 };
            }
            (0x8, _, _, 0x5) => {
                // Set Vx = Vx - Vy, set VF = carry.
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = new_vx;
                self.v[0xF] = if borrow { 0 } else { 1 };
            }
            (0x8, _, _, 0x6) => {
                // Set Vx = Vx SHR 1
                let x = digit2 as usize;
                let lsb = self.v[x] & 1;
                self.v[x] >>= 1;
                self.v[0xF] = lsb;
            }
            (0x8, _, _, 0x7) => {
                // Set Vx = Vx - Vy, set VF = carry.
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (new_vx, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = new_vx;
                self.v[0xF] = if borrow { 0 } else { 1 };
            }
            (0x8, _, _, 0xE) => {
                // Set Vx = Vx SHL 1
                let x = digit2 as usize;
                let msb = (self.v[x] >> 7) & 1;
                self.v[x] <<= 1;
                self.v[0xF] = msb;
            }
            (0x9, _, _, 0x0) => {
                // Skip the next instruction if Vx = Vy
                let x = digit2;
                let y = digit3;
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                // Set I = nnn
                let nnn: u16 = op & 0x0FFF;
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                // Jump to location nnn + V0.
                let nnn: u16 = op & 0x0FFF;
                self.pc = (self.v[0] as u16) + nnn;
            }
            (0xC, _, _, _) => {
                // Set Vx = random byte AND kk.
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                let rng: u8 = random();
                self.v[x] = rng & nn;
            }
            (0xD, _, _, _) => {
                //  Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                let x_coor = self.v[digit2 as usize] as u16;
                let y_coor = self.v[digit3 as usize] as u16;
                let height = digit4;

                // keep track if any pixels gets flipped
                let mut flipped = false;

                for y_line in 0..height {
                    let addr = self.i + y_line;
                    let pixels = self.ram[addr as usize];
                    for x_line in 0..8 {
                        let pixel = pixels & (0b1000_0000 >> x_line);
                        if pixel != 0 {
                            let x = (x_coor + x_line) as usize % DISPLAY_WIDTH;
                            let y = (y_coor + y_line) as usize % DISPLAY_HEIGHT;

                            // Get index of pixel to 1D
                            let index = x + DISPLAY_WIDTH * y;
                            flipped |= self.display[index];
                            self.display[index] ^= true;
                        }
                    }
                }
                self.v[0xF] = flipped as u8;
            }
            (0xE, _, 0x9, 0xE) => {
                // Skip next instruction if key with the value of Vx is pressed.
                let x = digit2 as usize;
                let vx = self.v[x];
                if self.keys[vx as usize] {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 0x1) => {
                //  Set Vx = delay timer value.
                let x = digit2 as usize;
                let vx = self.v[x];
                if !self.keys[vx as usize] {
                    self.pc += 2;
                }
            }
            (0xF, _, 0x0, 0x7) => {
                //  Set Vx = delay timer value.
                let x = digit2 as usize;
                self.v[x] = self.dt;
            }
            (0xF, _, 0x0, 0xA) => {
                // Wait for a key press, store the value of the key in Vx.
                let x = digit2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // Wait for input
                    self.pc -= 2;
                }
            }
            (0xF, _, 0x1, 0x5) => {
                // Set delay timer = Vx
                let x = digit2 as usize;
                self.dt = self.v[x];
            }
            (0xF, _, 0x1, 0x8) => {
                // Set sound timer = Vx
                let x = digit2 as usize;
                self.st = self.v[x];
            }
            (0xF, _, 0x1, 0xE) => {
                // Set I = I + Vx.
                let x = digit2 as usize;

                let value = self.v[x] as u16;
                self.i = self.i.wrapping_add(value);
            }
            (0xF, _, 0x2, 0x9) => {
                // Set I = location of sprite for digit Vx
                let x = digit2 as usize;
                let vx = self.v[x] as u16;
                self.i = vx * 5;
            }
            (0xF, _, 0x3, 0x3) => {
                // Store BCD representatioon of Vx in memory locations I, I+1, I+2
                let x = digit2 as usize;
                let vx = self.v[x] as f32;
                // The interpreter takes the decimal value of Vx, and places the
                // hundreds digit in memory at location in I, the tens digit at
                // location I+1, and the ones digit at location I+2.
                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.ram[self.i as usize] = hundreds;
                self.ram[(self.i + 1) as usize] = tens;
                self.ram[(self.i + 2) as usize] = ones;
            }
            (0xF, _, 0x5, 0x5) => {
                // Store registers V0 through Vx in memory starting at location I.
                let x = digit2 as usize;
                let i = self.i as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v[idx];
                }
            }
            (0xF, _, 0x6, 0x5) => {
                // Read registers V0 through Vx from memory starting at location I.
                let x = digit2 as usize;
                let i = self.i as usize;
                for idx in 0..=x {
                    self.v[idx] = self.ram[i + idx];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:#02X}", op),
        }
    }
}
