# Chip-8 Emulator

## Chip-8 Technical Specifications

- Memory: Chip-8 has direct access to up to 4 kilobytes of RAM
- Display: 64x32 pixels monochrome, i.e black or white
- A _program counter_, often called just "PC", whch points at the current instruction in memory.
- One 16-bit _index register_ called "I" which is used to point at locations in memory.
- A _stack_ for 16-bit addresses, which is used to call subroutines/functions and return from them.
- An 8-bit _delay timer_ which is decremented at a rate of 60Hz (60 times per second) until it reaches 0.
- An 8-bit _sound timer_ which functions like the delay timer, but which also gives off a beeping sound as long as it's not 0.
- 16 8-bit general-purpose _variable registers_ numbered 0 through F hexadecimal, called V0-VF
  - VF is also used as a flag reister; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag.