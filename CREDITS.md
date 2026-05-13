# Credits

A huge thanks to everyone who's shared their knowledge, projects, and research
with the Game Boy community; this project wouldn't exist without you. I'm
grateful to all the people and projects outlined here (and to the many more
influences I assuredly miss in this document) for making it fun and easy to
learn about and develop the Game Boy. Your contributions make the community
stronger and keep this classic system alive.

## References

This project would not have been possible without the countless Game Boy
community resources. Of these, I would like to specifically recognize the [Game
Boy Development community][gbdev].

[gbdev]: https://gbdev.io

See the list of resources (in no particular order) used as research for this
project below.

### Documentation

- [Pan Docs][pandocs]: Go-to community resource documenting the inner workings
  of the Game Boy. "The single most comprehensive technical reference to Game
  Boy available to the public."
- [Game Boy Architecture by Rodrigo Copetti][gbarch]: High-level practical
  analysis of the Game Boy.
  - Includes a helpful introduction to the PPU rendering pipeline.
- [Game Boy: Complete Technical Reference][gbctr]: Summation of [Gekkio]'s
  comprehensive Game Boy research.
  - Used for exact instruction timing breakdown.
- [The Gameboy Emulator Development Guide][gbedg]: Documentation intended to
  assist with development of emulators for the original DMG Game Boy.
  - Used extensively for the initial PPU and timer implementations.
- [Nitty Gritty Gameboy Cycle Timing][nitty]: Down and dirty timing of the Game
  Boy's video hardware.
- [GhostSonic's Sound Emulation Comment][gsonic]: Detailed comment discussing
  implementing and emulating the APU.
- [Game Boy Sound Emulation][ns256]: A short article on the Game Boy sound
  hardware with the perspective of emulating it.

[gbarch]:  https://www.copetti.org/writings/consoles/game-boy
[gbctr]:   https://gekkio.fi/files/gb-docs/gbctr.pdf
[gbedg]:   https://hacktix.github.io/GBEDG/
[gekkio]:  https://gekkio.fi
[gsonic]:  https://www.reddit.com/r/EmuDev/comments/5gkwi5/comment/dat3zni
[nitty]:   http://blog.kevtris.org/blogfiles/Nitty%20Gritty%20Gameboy%20VRAM%20Timing.txt
[ns256]:   https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html
[pandocs]: https://gbdev.io/pandocs/

### Hardware

- [Emu-Russia's DMG-01 SM83 Core Research][dmgcpu]: Verilog model with
  invaluable accompanying diagrams of the SM83 core.

[dmgcpu]: https://github.com/emu-russia/dmgcpu

## Attribution

This project uses and distributes the following open-source software under the
conditions of their respective licenses:

### Firmware

- [SameBoy's Boot ROM](/roms/boot/sameboy/dmg_boot.bin) is included under the
  conditions of the [MIT License](/roms/boot/sameboy/LICENSE) (dated 29 Aug
  2023). See the project [here][sameboy].

[sameboy]: https://sameboy.github.io

### Games

- [2048](/roms/games/2048/2048.gb) is included under the conditions of the
  [zlib License](/roms/games/2048/LICENSE) (dated 29 Aug 2023). See the
  project [here][2048].
- [Porklike](/roms/games/porklike/porklike.gb) is included under the
  conditions of the [MIT License](/roms/games/porklike/LICENSE) (dated 16 Oct
  2024). See the project [here][porklike].

[2048]:     https://github.com/Sanqui/2048-gb
[porklike]: https://github.com/binji/porklike.gb

### Testing

- [Blargg's Test Suite](/roms/test/blargg) is included under presumptive
  permissive licensing, though no explicit license could be found. See the
  project [here][blargg].
- [dmg-acid2 Test ROM](/roms/test/acid2/dmg-acid2.gb) is included under the
  conditions of the [MIT License](/roms/test/acid2/LICENSE) (dated 08 Jan
  2024). See the project [here][acid2].
- [Mealybug Tearoom Tests](/roms/test/mealybug) is included under the
  conditions of the [MIT License](/roms/test/mealybug/LICENSE) (dated 21 Apr
  2024). See the project [here][mealybug].
- [Mooneye Test Suite](/roms/test/mooneye) is included under the conditions of
  the [MIT License](/roms/test/mooneye/LICENSE) (dated 06 Sep 2023). See the
  project [here][mooneye].

[acid2]:    https://github.com/mattcurrie/dmg-acid2
[blargg]:   https://github.com/retrio/gb-test-roms
[mealybug]: https://github.com/mattcurrie/mealybug-tearoom-tests
[mooneye]:  https://github.com/Gekkio/mooneye-test-suite

### Utilities

- [which.gb](/roms/util/which/which.gb) is included under the conditions of the
  [MIT License](/roms/util/which/LICENSE) (dated 12 May 2026). See the project
  [here][which].

[which]: https://github.com/mattcurrie/which.gb
