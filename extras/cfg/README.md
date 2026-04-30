# rugby-cfg

This library contains (de)serializable configuration schema usable by various
`rugby` frontends. This is intended to allow for configuration files to be
shared between frontends enabling consistent behaviour throughout.

## Hierarchy

Configurable fields are arranged hierarchically as such:

```
.
├── log:   string
├── audio: object
│  └── rate: uint
├── video: object
│  └── palette: enum
├── input: object
├── cable: object
├── boot:  object
│  └── rom:  path
└── cart:  object
   ├── rom:   path
   ├── check: bool
   ├── force: bool
   └── save:  enum
```

## Options

### Fields

The following is a table of supported configurable fields:

| Field        | Description                   | Flag            | Type     | Clap  | Serde | Notes  |
|--------------|-------------------------------|-----------------|----------|:-----:|:-----:|--------|
| `log`        | Logging filter.               | `-l/--log`      | `string` |   ✓   |   ✓   | [^log] |
| `audio.rate` | Audio sample rate.            | `--sample-rate` | `uint`   |   ✓   |   ✓   | [^aux] |
| `video.pal`  | 2-bit color palette.          | `-p/--palette`  | `enum`   |   ✓   |   ✓   | [^pal] |
| `boot.rom`   | Boot ROM image file.          | `-b/--boot`     | `path`   |   ✓   |   ✓   |        |
| `cart.rom`   | Cartridge ROM image file.     |                 | `path`   |   ✓   |       |        |
| `cart.check` | Check cartridge integrity.    | `-c/--check`    | `bool`   |   ✓   |   ✓   |        |
| `cart.force` | Force cartridge construction. | `-f/--force`    | `bool`   |   ✓   |   ✓   |        |
| `cart.save`  | Cartridge RAM persistence.    | `-S/--save`     | `enum`   |   ✓   |   ✓   | [^sav] |

[^aux]: Unless you have a specific use case, there is no reason to change the
    default value of 48 KHz.
[^log]: Must be a valid log filter as parsed by the frontend. See filter
    directives using [`tracing`][filter] as an example.
[^pal]: Only applicable on the DMG model. On CGB, the palette will be ignored.
[^sav]: Specifies when the cartridge RAM should be loaded/saved to disk.

### Types

All the expected primitive types for options are supported:

- `bool`: boolean value, specified as `true` or `false`.
- `float`: floating-point number, can be in scientific notation.
- `[u]int`: integer number, may be unsigned.
- `string`: string of characters encoded in UTF-8.

Additionally, there are more advanced types:

- `enum`: enumerated choice from a predefined set of values.
- `path`: filesystem path to a file or directory.

These may be combined into an `object`, which is simply a collection of fields.

### Enums

All enumerated types are described below:

- `palette`: color palette selection, see [variants][src.pal]; can be customized
  as an array of 4 colors (parsed in hex).
- `speed`: simulated clock frequency, see [variants][src.spd]; can be specified
  as:
  - `actual`: actual hardware speed
  - `<mult>x`: speedup ratio; e.g. `x = 1.5`
  - `<freq>hz`: clock frequency; e.g. `hz = 6291456`
  - `<rate>fps`: frame rate; e.g. `fps = 90`
  - `turbo`: maximum possible speed
- `when`: choice of when to enable an option, [variants][src.when] are: `never`,
  `auto`, and `always`.

## License

For information regarding licensure, please see the project's [README][license].

<!-- Reference-style links -->
[filter]:   https://tracing.rs/tracing_subscriber/filter/struct.envfilter#directives
[license]:  /README.md#license
[src.pal]:  ./src/group/video.rs#L21
[src.spd]:  ./src/types/speed.rs#L22
[src.when]: ./src/types/mod.rs#L22
