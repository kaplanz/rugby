# rugby-cli

[![supports Linux][nix.badge]](#)
[![supports macOS][mac.badge]](#)
[![supports Windows][win.badge]](#)

This frontend provides a command-line interface (CLI) to quickly launch an
emulator session right from your terminal. It has first-class support for all
main features, including a full interactive debugger.

## Build

Invoke `cargo build --release -prugby-cli` from anywhere within the repository to
build this frontend. The following artifacts of interest will be produced:

```
./target/release/
├── rugby          # rugby-cli executable
├── ...
└── build/rugby-cli-*/out/
   ├── _rugby      # completions for zsh
   ├── _rugby.ps1  # completions for powershell
   ├── rugby.6     # manual documentation page
   ├── rugby.bash  # completions for bash
   ├── rugby.elv   # completions for elvish
   └── rugby.fish  # completions for fish
```

## Usage

### Command-line

To see usage information, run the `rugby` executable with the `-h` flag (use
`--help` for more detailed descriptions):

```
Emulate the Nintendo Game Boy

Usage: rugby [OPTIONS] [ROM]

Arguments:
  [ROM]  Cartridge ROM image file

Options:
      --conf <PATH>  Configuration file [env: RUGBY_CONF=]
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version

Runtime:
  -x, --exit      Exit without running
  -H, --headless  Run in headless mode

Startup:
  -l, --log <FILTER>     Logging filter [env: RUGBY_LOG=]
  -p, --palette <COLOR>  2-bit color palette [possible values: autumn-chill,
                         blk-aqu, blue-dream, coldfire, coral, demichrome,
                         earth, ice-cream, legacy, mist, mono, morris,
                         purple-dawn, rustic, velvet-cherry]
  -s, --speed <FREQ>     Simulated clock speed [possible values: half, actual,
                         double, max]

Console:
  -b, --boot [<PATH>]  Boot ROM image file
  -c, --check          Check cartridge integrity
  -f, --force          Force cartridge construction
  -S, --save <WHEN>    Cartridge RAM persistence [possible values: never, auto,
                       always]

Serial:
      --host <ADDR>  Link cable local address
      --peer <ADDR>  Link cable peer address

Debug:
  -i, --gbd              Enable interactive debugging
      --trace <FORMAT>   Enable introspective tracing [possible values: binjgb,
                         doctor]
      --tracelog <PATH>  Output tracing logfile
      --win              Enable VRAM debug windows
```

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

The default configuration is as follows:

```toml
[app]
# Logging level
log = "warn"
# 2-bit color palette
#
# Value must be a named preset or customized as follows:
# palette = { custom = ["#222", "#666", "#aaa", "#ddd"] }
palette = "mono"
# Simulated clock speed
#
# Value must be a named preset or customized as follows:
# speed   = { fps = 90 }      # runs at 1.50x
# speed   = { hz  = 3145728 } # runs at 0.75x
speed = "actual"

[emu.cart]
# Check cartridge integrity
check = false
# Force cartridge construction
force = false
# Cartridge RAM persistence
save = "auto"

[emu.boot]
# Boot ROM image file
# rom = "path/to/dmg_boot.bin"
```

A customized example could be found [here][config].

### Debugging

If `rugby` is launched with `-i/--gbd`, the program will instead present the
Game Boy Debugger (GBD) prompt after initialization. In this mode, commands can
be run to control and monitor execution of the console. While running with GBD
enabled, CTRL-C could be used to interrupt emulation and present the prompt.

To list and get help with GBD, use the `help` command at the prompt or see its
[documentation](/gbd/README.md).

## Progress

- [x] Static configuration
  - [x] Palette customization
  - [x] Frequency selection
  - [ ] Re-mappable joypad
- [x] Debugging support
  - [x] Interactive debugging
  - [x] CPU state logging
  - [x] PPU VRAM visualizer

## License

For information regarding licensure, please see the project's [README][license].

<!--
  Reference-style links
-->

<!-- Badges -->
[mac.badge]: https://img.shields.io/badge/macOS-000?logo=apple&logoColor=fff
[nix.badge]: https://img.shields.io/badge/Linux-FCC624?logo=linux&logoColor=000
[win.badge]: https://img.shields.io/badge/Windows-0078D4?logo=windows&logoColor=fff

<!-- Usage -->
[config]:  ./docs/config.toml

<!-- License -->
[license]: /README.md#license
