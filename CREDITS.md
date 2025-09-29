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

### Hardware

- [Emu-Russia's DMG-01 SM83 Core Research][dmgcpu]: Verilog model with
  invaluable accompanying diagrams of the SM83 core.

## Attribution

This project uses and distributes the following open-source software under the
conditions of their respective licenses:

### Firmware

- [SameBoy's Boot ROM][sameboy.boot] is included under the conditions of the
  [MIT License][sameboy.license] (dated 29 Aug 2023). See the project
  [here][sameboy].

### Games

- [2048][2048.game] is included under the conditions of the
  [zlib License][2048.license] (dated 29 Aug 2023). See the project
  [here][2048].
- [Porklike][porklike.game] is included under the conditions of the
  [MIT License][porklike.license] (dated 16 Oct 2024). See the project
  [here][porklike].

### Testing

- [Blargg's Test Suite][blargg.test] is included under presumptive permissive
  licensing, though no explicit license could be found. See the project
  [here][blargg].
- [dmg-acid2 Test ROM][acid2.test] is included under the conditions of the [MIT
  License][acid2.license] (dated 08 Jan 2024). See the project [here][acid2].
- [Mealybug Tearoom Tests][mealybug.test] is included under the conditions of
  the [MIT License][mealybug.license] (dated 21 Apr 2024). See the project
  [here][mealybug].
- [Mooneye Test Suite][mooneye.test] is included under the conditions of the
  [MIT License][mooneye.license] (dated 06 Sep 2023). See the project
  [here][mooneye].

<!--
  Reference-style links
-->

<!-- References -->
[dmgcpu]:    https://github.com/emu-russia/dmgcpu
[gbarch]:    https://www.copetti.org/writings/consoles/game-boy
[gbctr]:     https://gekkio.fi/files/gb-docs/gbctr.pdf
[gbdev]:     https://gbdev.io
[gbedg]:     https://hacktix.github.io/GBEDG/
[gekkio]:    https://gekkio.fi
[gsonic]:    https://www.reddit.com/r/EmuDev/comments/5gkwi5/comment/dat3zni
[nitty]:     http://blog.kevtris.org/blogfiles/Nitty%20Gritty%20Gameboy%20VRAM%20Timing.txt
[ns256]:     https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html
[pandocs]:   https://gbdev.io/pandocs/

<!-- Attribution -->
[2048]:             https://github.com/Sanqui/2048-gb
[2048.game]:        ./roms/games/2048/2048.gb
[2048.license]:     ./roms/games/2048/LICENSE
[acid2]:            https://github.com/mattcurrie/dmg-acid2
[acid2.doc]:        ./tests/README.md#acid2
[acid2.test]:       ./roms/test/acid2/dmg-acid2.gb
[acid2.license]:    ./roms/test/acid2/LICENSE
[blargg]:           https://github.com/retrio/gb-test-roms
[blargg.doc]:       ./tests/README.md#blargg
[blargg.test]:      ./roms/test/blargg
[mealybug]:         https://github.com/mattcurrie/mealybug-tearoom-tests
[mealybug.doc]:     ./tests/README.md#mealybug
[mealybug.test]:    ./roms/test/mealybug
[mealybug.license]: ./roms/test/mealybug/LICENSE
[mooneye]:          https://github.com/Gekkio/mooneye-test-suite
[mooneye.doc]:      ./tests/README.md#mooneye
[mooneye.test]:     ./roms/test/mooneye
[mooneye.license]:  ./roms/test/mooneye/LICENSE
[porklike]:         https://github.com/binji/porklike.gb
[porklike.game]:    ./roms/games/porklike/porklike.gb
[porklike.license]: ./roms/games/porklike/LICENSE
[sameboy]:          https://sameboy.github.io
[sameboy.boot]:     ./roms/boot/sameboy/dmg_boot.bin
[sameboy.license]:  ./roms/boot/sameboy/LICENSE
