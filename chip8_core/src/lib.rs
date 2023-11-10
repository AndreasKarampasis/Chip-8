const RAM_SIZE: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
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
}
