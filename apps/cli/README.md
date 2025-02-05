# rugby-cli

[![supports Linux][nix.badge]](#)
[![supports macOS][mac.badge]](#)
[![supports Windows][win.badge]](#)

This frontend provides a command-line interface (CLI) to quickly launch an
emulator session right from your terminal. It has first-class support for all
main features, including a full interactive debugger.

## Build

Invoke `cargo build --release -prugby-cli` from anywhere within the repository
to build this frontend.

## Usage

### Commands

As a single-binary application, `rugby` provides several useful commands
(outlined below). To see usage information, run the executable with the `-h`
flag (use `--help` for more detailed descriptions).

#### `check`

Attempts to construct the provided ROM without performing any emulation. On
success, ROM information is printed to the console.

#### `run`

Emulates the provided ROM. See options to customize emulation behaviour
[below](#configuration).

#### `gen`

Generates static files for the command-line application which are printed to
console. Currently implements the following subcommands:

- `cfg`: Configuration file.
- `cmp`: Shell completions.
- `cfg`: Manual pages.

> [!NOTE]
>
> Save the contents of a generated file by piping the output of the command,
> e.g.:
>
> ```
> rugby gen cfg > ~/.config/rugby/config.toml
> ```

#### `help`

Opens the manual page for the corresponding command. Can also be specified using
the alias `man`.

### Configuration

When run, the program will load persistent configuration options from the first
file found according to following precedence rules:
1. Command-line option `--conf=<PATH>`.
1. Environment variable `RUGBY_CONF`.
1. Default path `$XDG_CONFIG_HOME/rugby/config.toml`.

When options are specified in multiple locations, they will be applied with the
following precedence: cli > env > file. This means these options may be
overridden when running the program.

Any relative paths specified in this file are resolved relative to this file's
parent directory. Use `--conf` on the command-line to provide an alternate
location for this file.

The default configuration could either be found [here](./rugby.toml), or
generated with:

```
rugby gen cfg
```

> [!NOTE]
>
> Configuration options are [documented][cfg.doc] in the `rugby-cfg` crate.

### Debugging

If `rugby` is launched with `-i/--gbd`, the program will instead present the
Game Boy Debugger (GBD) prompt after initialization. In this mode, commands can
be run to control and monitor execution of the console. While running with GBD
enabled, CTRL-C could be used to interrupt emulation and present the prompt.

To list and get help with GBD, use the `help` command at the prompt or see its
[documentation][gbd.doc].

## Progress

- [x] Emulator
  - [x] Audio
  - [x] Joypad
    - [x] Keyboard inputs
    - [ ] Custom bindings
  - [x] Video
    - [x] Custom palettes
- [x] Frontend
  - [x] App settings
    - [x] Configuration file
  - [x] Command-line interface
    - [x] Generate static files
      - [x] Configuration file
      - [x] Shell completions
      - [x] Manual pages
  - [x] Features
    - [x] Change clock speed
- [x] Debugging
  - [ ] Dynamic TUI application
  - [x] Interactive debug prompt
    - [x] Peek/poke memory
    - [x] Peek/poke registers
  - [x] CPU state log tracing
  - [x] Video RAM visualizer

## License

For information regarding licensure, please see the project's [README][license].

<!-- Reference-style links -->
[cfg.doc]: /apis/cfg/README.md
[gbd.doc]: /apis/gbd/README.md
[license]: /README.md#license

<!-- Badges -->
[mac.badge]: https://img.shields.io/badge/macOS-000?logo=apple&logoColor=fff
[nix.badge]: https://img.shields.io/badge/Linux-FCC624?logo=linux&logoColor=000
[win.badge]: https://img.shields.io/badge/Windows-0078D4?logo=windows&logoColor=fff
