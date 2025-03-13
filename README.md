<h1 align="center">
  <img width="128" height="128" src="./docs/assets/img/icon.png"/>
  <p>Rugby</p>
</h1>

<p align="center">
  A cycle accurate emulator of the original 1989 Nintendo Game Boy.
</p>

[![dependencies][dep.badge]][dep.hyper]
[![cli frontend][cli.badge]](./apps/cli)
[![web frontend][web.badge]](./apps/web)

> [!IMPORTANT]
>
> Consider this to be pre-alpha software. I make no guarantee of the stability
> of any documented APIs until the first official release.

## Goal

This project aims to provide high-accuracy emulation of all major components in
the SM83-based Nintendo Game Boy family of consoles. Each component is modular,
enabling them to be easily substituted for one another. In turn, this leads to
emulator instances supporting multiple implementations of core components with
different accuracy/performance margins.

> [!NOTE]
>
> At this time, there are no plans to support the later Game Boy Advance, which
> has an entirely different architecture.

## Organization

In accordance with the preference for modularity outlined above, the project is
partitioned into the [core](./core), the [debugger](./apis/gbd), and various
emulator [frontends](./apps). Useful supporting [crates](./crates) are
independently available as well. Also packaged in this repository are several
open-source [ROMs](./roms). These are used internally for testing and to demo
project functionality.

### Workspace

Cargo — Rust's package manager — allows for a workspace of several crates to be
specified within its [manifest](./Cargo.toml). Within this project, workspace
crates are used with the structure as follows:

```
./
├── Cargo.lock       # cargo lockfile
├── Cargo.toml       # cargo manifest
├── Justfile         # useful dev commands
├── README.md        # this document
├── ...
├── apis/            # app interfaces
│  ├── cfg/          # app configuration
│  ├── gbd/          # game boy debugger
│  └── pal/          # palette presets
├── apps/            # frontend apps
│  ├── cli/          # command-line app
│  └── web/          # web assembly app
├── arch/            # arch primitives
├── core/            # emulation core
├── crates/          # support crates
│  ├── chex/         # color hex parser
│  ├── hexd/         # hex dump printer
│  └── wrange/       # wrapping ranges
├── docs/            # documentation
├── examples/        # example frontends
├── lib/             # library frontends
│  └── retro/        # libretro port
├── roms/            # open-source ROMs
│  ├── boot/         # boot ROM images
│  ├── games/        # playable demos
│  └── test/         # test ROM images
├── src/             # top-level library
└── tests/           # integration tests
```

> [!TIP]
>
> For downstream library users, I recommend using the top-level `rugby` crate
> rather than directly using `rugby-core`, as it better structured for end
> users and includes useful supporting modules.

### Components

Main components of `rugby` are outlined below.

- [`rugby`](./): is the high-level library that provides the public API.
- [`rugby-arch`](./arch): defines the architecture primitives for emulation.
- [`rugby-core`](./core): contains the implementation of emulator cores.

#### Interfaces

- [`rugby-cfg`](./apis/cfg): definition of application configuration options.
- [`rugby-gbd`](./apis/gbd): provides an interactive prompt debugging interface.
- [`rugby-pal`](./apis/pal): a collection of DMG palette presets.

#### Frontends

- [`rugby-cli`](./apps/cli): a command-line interface application with rich
  configuration support and comprehensive debugging options.
- [`rugby-web`](./apps/web): an online web application powered by [Wasm].

Several additional frontends are planned for the future:

- [`rugby-ios`](./apps/ios): a native iOS application built with [SwiftUI].
- ~`rugby-sdl`: a cross-platform application based upon the [SDL] framework.~
  - No longer planned, use `rugby-cli` or `libretro` instead.

#### Libraries

- [`libretro`](./lib/retro): implementation of the libretro API.

## [Testing](./tests/README.md)

Rigorous integration testing is provided to validate the implementation and
prevent regressions in future versions.

|   Suite                  | Passed | Failed |
| ------------------------ | ------ | ------ |
| [Acid2][acid2.doc]       |     17 |      0 |
| [Blargg][blargg.doc]     |     23 |     22 |
| [Mealybug][mealybug.doc] |      0 |     24 |
| [Mooneye][mooneye.doc]   |     30 |     39 |

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

## License

This project is dual-licensed under both [MIT License](./LICENSE-MIT) and
[Apache License 2.0](./LICENSE-APACHE). You have permission to use this code
under the conditions of either license pursuant to the rights granted by the
chosen license.

<!--
  Reference-style links
-->

<!-- Badges -->
[cli.badge]: https://img.shields.io/badge/frontend-cli-blue
[dep.badge]: https://deps.rs/repo/github/kaplanz/rugby/status.svg
[dep.hyper]: https://deps.rs/repo/github/kaplanz/rugby
[web.badge]: https://img.shields.io/badge/frontend-web-orange

<!-- Organization -->
[sdl]:     https://www.libsdl.org
[swiftui]: https://developer.apple.com/xcode/swiftui/
[wasm]:    https://webassembly.org

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
