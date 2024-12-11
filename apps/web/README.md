# [rugby-web][this]

[![bun runtime][bun.badge]][bun.hyper]
[![lit element][lit.badge]][lit.hyper]
[![webassembly][wasm.badge]][wasm.hyper]

This frontend is a web application that runs entirely on-browser, enabling it to
be hosted as static files (no server code). It supports uploading games from a
file, which persist locally as temporary storage (similar to cookies). You can
play use it live [here][this].

## Build

Ensure [Bun][bun.hyper] is installed, then run `bun install` to install
dependencies. Afterwards, the application can be built with `bun run build`.

> [!NOTE]
>
> The build scripts will automatically compile the core to Wasm. You can do this
> manually by running `wasm-pack build`.

### Development

To run locally during development, install dependencies (see above) then start
the development server with `bun run dev`.

## Usage

### Controls

Emulator controls should be fairly self-explanatory, with on-screen buttons
corresponding to their emulator inputs. You can toggle power by clicking the
"OFF &bullet; ON" label on the frame. Open the menu clicking the
(&#x2139;&#xfe0e;) button.

### Bindings

In addition to the on-screen buttons, these key bindings can be used as joypad
inputs:

| Keyboard | Emulator |
|----------|----------|
| X        | A        |
| Z        | B        |
| Enter    | Start    |
| Space    | Select   |
| Arrows   | D-pad    |

There are also key bindings for the application:

| Keyboard | Action        |
|----------|---------------|
| ?        | Toggle menu   |
| F1       | Show menu     |
| Escape   | Hide menu     |
| 1-9      | Open menu tab |

## Progress

- [x] Emulator
  - [x] Joypad
    - [x] Keyboard events
    - [x] Mouse events
    - [x] Touch events
  - [x] Screen
    - [ ] Custom palettes
- [x] Frontend
  - [x] Documentation
    - [x] About panel
    - [x] Usage panel
  - [x] Game library
    - [x] Upload ROM files
    - [x] Persistent storage
    - [ ] Save RAM snapshots
  - [x] App settings
    - [x] Play/pause emulator
    - [x] Change clock speed

## License

For information regarding licensure, please see the project's [README][license].

<!--
  Reference-style links
-->

[this]:  https://rugby.zakhary.dev

<!-- Badges -->
[bun.badge]:  https://img.shields.io/badge/Bun-black?logo=bun&logoColor=f9f1e1
[bun.hyper]:  https://bun.sh
[lit.badge]:  https://img.shields.io/badge/Lit-334eff?logo=lit
[lit.hyper]:  https://lit.dev
[wasm.badge]: https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=white
[wasm.hyper]: https://webassembly.org

<!-- License -->
[license]: /README.md#license
