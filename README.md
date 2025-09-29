<h1 align="center">
  <img width="128" height="128" src="./site/www/img/logo.png"/>
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
partitioned into the [core](./core), the [debugger](./extras/gbd), and various
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
├── apps/            # frontend apps
│  ├── cli/          # command-line app
│  ├── ios/          # SwiftUI iOS app
│  └── web/          # WebAssembly app
├── arch/            # arch primitives
├── bind/            # language bindings
│  ├── swift/        # Swift (UniFFI)
│  └── wasm/         # WebAssembly
├── core/            # emulation core
├── crates/          # support crates
│  ├── bfmt/         # bytes formatter
│  ├── chex/         # color hex parser
│  ├── hexd/         # hex dump printer
│  └── orng/         # wrapping ranges
├── docs/            # documentation
├── examples/        # example frontends
├── extras/          # extra emulator APIs
│  ├── cfg/          # app configuration
│  ├── gbd/          # game boy debugger
│  └── pal/          # palette presets
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

### Modules

Main components of `rugby` are outlined below.

- [`rugby`](./): is the high-level library that provides the public API.
- [`rugby-arch`](./arch): defines the architecture primitives for emulation.
- [`rugby-core`](./core): contains the implementation of emulator cores.

#### Extras

- [`rugby-cfg`](./extras/cfg): definition of app configuration options.
- [`rugby-gbd`](./extras/gbd): provides an interactive prompt debugger.
- [`rugby-pal`](./extras/pal): collection of DMG color palette presets.

#### Frontends

- [`rugby-cli`](./apps/cli): a command-line interface application with rich
  configuration and comprehensive debugging.
- [`rugby-ios`](./apps/ios): a native iOS application built with [SwiftUI].
- [`rugby-web`](./apps/web): an online web application powered by [Wasm].

#### Bindings

- [`rugby-swift`](./bind/swift): foreign function interface to Swift (UniFFI).
- [`rugby-wasm`](./bind/wasm): cross-compilation with bindings to WebAssembly.

#### Libraries

- [`libretro`](./lib/retro): implementation of the libretro API.

## [Testing]

Rigorous integration testing is provided to validate the implementation and
prevent regressions in future versions.

|   Suite                   | Passed | Failed |
| ------------------------- | ------ | ------ |
| [Acid2][test.acid2]       |     17 |      0 |
| [Blargg][test.blargg]     |     23 |     22 |
| [Mealybug][test.mealybug] |      0 |     24 |
| [Mooneye][test.mooneye]   |     30 |     39 |

## Credits

This project would not have been possible without the people, projects, and
resources that contributed to my learning and make development so fun and easy.

See the [Credit Roll](./CREDITS.md) for a full list.

## Privacy

The project does not collect or transmit any data. Any data used by the project
remains solely on your device.

See the [Privacy Policy](./PRIVACY.md) for full details.

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
[swiftui]: https://developer.apple.com/xcode/swiftui/
[wasm]:    https://webassembly.org

<!-- Testing -->
[testing]:       ./tests/README.md
[test.acid2]:    ./tests/README.md#acid2
[test.blargg]:   ./tests/README.md#blargg
[test.mealybug]: ./tests/README.md#mealybug
[test.mooneye]:  ./tests/README.md#mooneye
