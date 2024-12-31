# rugby-core

This library implements the Game Boy emulator core. As the internal
implementation library, it is not intended to be used directly, rather it should
be used through the top-level [`rugby`](/) crate.

Currently, only the original Game Boy (DMG) is supported, although future
support for the Game Boy Color (CGB) is planned.

## Progress

- [x] Implementation
  - [ ] Audio (APU)
  - [x] Cartridge
    - [x] Save RAM to disk
    - [ ] Supported kinds
      - [x] ROM
      - [x] MBC1
      - [ ] MBC2
      - [x] MBC3
      - [x] MBC5
      - [ ] Camera
  - [x] Interrupts (PIC)
  - [x] Graphics (PPU)
    - [x] Functional correctness
      - [x] Background drawing
      - [x] Window overlay
      - [x] Sprite rendering
    - [ ] Implementation accuracy
  - [x] Joypad
  - [x] Processor (CPU)
    - [x] Functional correctness
    - [ ] Implementation accuracy
      - [x] Multi-cycle instructions
      - [x] Timed memory accesses
      - [ ] Precise cycle timings
  - [x] Serial
  - [x] Timer
    - [x] Functional correctness
    - [x] Implementation accuracy
- [ ] Performance
  - [x] Real-time emulation
  - [ ] Benchmark tests

## License

For information regarding licensure, please see the project's [README][license].

<!-- Reference-style links -->
[license]: /README.md#license
