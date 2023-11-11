# Chip-8 Emulator

## Chip-8 Technical Specifications

- Memory: Chip-8 has direct access to up to 4 kilobytes of RAM
- Display: 64x32 pixels monochrome, i.e black or white
- A *program counter*, often called just "PC", whch points at the current instruction in memory.
- One 16-bit *index register* called "I" which is used to point at locations in memory.
- A *stack* for 16-bit addresses, which is used to call subroutines/functions and return from them.
- An 8-bit *delay timer* which is decremented at a rate of 60Hz (60 times per second) until it reaches 0.
- An 8-bit *sound timer* which functions like the delay timer, but which also gives off a beeping sound as long as it's not 0.
- 16 8-bit general-purpose *variable registers* numbered 0 through F hexadecimal, called V0-VF
  - VF is also used as a flag reister; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag.

## How to use

## Credits

- [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Tobias V. Langhoff's Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
