# libretro

> **[RetroArch]** is a free and open-source, cross-platform frontend for
> emulators, game engines, video games, media players and other applications. It
> is the reference implementation of the **[libretro]** API, designed to be fast,
> lightweight, portable and without dependencies.

[retroarch]: https://www.retroarch.com
[libretro]:  https://www.libretro.com

## Build

Invoke `cargo build --release -plibretro` from anywhere within the repository
to build this frontend.

## Usage

To use this the port, add the compiled dynamic library (usually called
`libretro.so`) to your local **RetroArch** installation. For more information,
consult **libretro**'s [documentation][docs].

> [!TIP]
>
> - On macOS, dynamic libraries use the `.dylib` extension.
> - On Windows, dynamic libraries use the `.dll` extension.

[docs]: https://docs.libretro.com

## Progress

- [x] Emulator
  - [x] Audio
  - [x] Cartridge
    - [ ] Save RAM to disk
  - [x] Joypad
    - [x] Controller support
  - [x] Video
    - [ ] Custom palettes
- [ ] Features
  - [ ] Network play
  - [ ] Save states

## License

For information regarding licensure, please see the project's [README][license].

<!-- Reference-style links -->
[license]: /README.md#license
