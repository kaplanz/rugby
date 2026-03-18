# Style

The following style guide outlines how this project structures it's source code
and documentation prose. If you are contributing to the project, please adhere
to the styles presented in this document.

## General

### Naming

- Names are concise; prefer short, meaningful abbreviations over long
  descriptive names when the context makes meaning clear:
  - E.g., `cfg`, `ctx`, `cxn`, `buf`, `len`, `idx`, `req`, `res`, `srv`.
- Acronyms and abbreviations follow the language's casing conventions rather
  than being forced to all-caps or all-lowercase.
- File and module names are lowercase, short, and avoid redundancy with their
  containing directory.

### Comments

There are two kinds of comments: **doc comments** (rendered by tooling) and
**impl comments** (explanatory, not rendered).

#### Doc comments

- Public items use complete sentences ending in a period.
- Fields and properties use fragments without a trailing period.
- Links are collected as reference-style links at the bottom of the file or
  module, not inline.

#### Impl comments

Comments explain *why*, not *what*. Avoid restating what the code already says.

Two styles:

**Short** -- a brief label, no period, fewer than ~5 words, not necessarily a
complete sentence. Used as section headers within a block of code.

```
// Parse config
cfg = parse(raw)

// Validate
validate(cfg)
```

**Long** -- a header line (same rules as short, but ending with a period),
followed by a blank comment line, then a full explanation in complete sentences
with full punctuation. Wraps at 80 columns. No em-dashes.

```
// Parse config.
//
// The config format allows partial overrides, so missing fields are
// filled in from defaults before validation runs.
cfg = parse(raw)
```

### Formatting

- Prefer spaces to tabs for indentation universally, unless tabs are required by
  language syntax. Usually prefer 4-space indentation.
- Line length: 100 characters for code, 80 for comments and prose.
- Trailing commas on multi-line constructs wherever the language allows.
- Opening braces on the same line; no Allman style.

---

## Commit Style

- [Conventional commits][cc]: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `perf`, `refactor`, `style`, `test`, `chore`,
  `build`, `revert`
- Subject line ≤ 64 characters, **not** capitalized, **not** imperative mood.
- Scope is optional but used when relevant (e.g., `feat(cli): ...`,
  `feat(chex): ...`).

[cc]: https://www.conventionalcommits.org

---

## Markdown

- Documents use reference-style links grouped at the bottom.
- Tables are padded with spaces for visual alignment.
- GitHub admonitions (`> [!NOTE]`, `> [!WARNING]`, `> [!TIP]`) are used for
  callouts in documentation.
- ASCII diagrams use Unicode box-drawing characters (─, │, ┌, ┐, └, ┘, ├, ┤,
  etc.).
- Headings follow a strict hierarchy; `#` is the document title, then `##`,
  `###`, etc.
- Code blocks in documentation include a language identifier.
- Prose wraps at 80 columns.

---

## Languages

### Rust

#### Formatting & linting

- `rustfmt` with nightly (`cargo +nightly fmt --all`).
- `#![warn(clippy::pedantic)]` in every crate root.
- Specific `#![allow(...)]` and `#[expect(...)]` suppressions at the narrowest
  possible scope; `#[expect]` is preferred over `#[allow]` for single-site
  suppressions so they become errors if the lint no longer fires.
- `#[rustfmt::skip]` used on blocks that are more readable with custom
  formatting (e.g., aligned match arms, large tables).

#### Module structure

- `mod.rs` declares and re-exports sub-modules; `imp.rs` holds the
  implementation details (`pub(super)`).
- Public items in `lib.rs` are re-exported with `pub use`.
- Implementation details are hidden in private modules and exposed selectively.

For a component `cpu`:

```
cpu/
  mod.rs    -- public API, types, trait impls
  imp.rs    -- implementation details (pub(super))
```

#### Types & traits

- New-types (`struct Freq(u32)`) are used to add behavior to foreign types
  rather than type aliases where possible.
- Type aliases simplify return and field types:
  `type Result<T, E = Error> = std::result::Result<T, E>;`
- Traits define the primary interface; concrete types implement them. Trait
  objects are boxed when needed for heterogeneity.
- `#[must_use]` on all pure constructors and getters that return a value.
- `Default` is derived rather than hand-implemented unless custom behavior is
  required.
- `Debug` is always derived or implemented; `Display` is implemented for
  user-facing types.

#### Structs

- Fields are documented with `///` comments (fragments, no period).
- Structs group related fields logically; field ordering within a struct follows
  a semantic grouping, not alphabetical.
- Visibility is minimal: `pub` only where necessary for the crate's API,
  `pub(crate)` or `pub(super)` otherwise.

  ```rust
  pub struct Bus {
      /// Address space.
      pub mem: Memory,
      /// Attached devices.
      dev: Vec<Box<dyn Device>>,
  }
  ```

#### Enums

- Enum variants are documented with `///` comments.
- Flag-style enums use explicit discriminant values written as binary literals
  with grouping underscores: `0b0000_0001`.
- Hex literals use lowercase: `0xff`, `0xfffe`.
- Numeric literals use underscore separators for readability: `4_194_304`.

#### Constructors

- `new()` for default construction (mirrors `Default`).
- `with(...)` for construction with a primary configuring argument.
- `from(...)` via `From` trait implementations.
- Constructors call `Self::default()` then mutate, or call a private
  `play()`/`init()` helper.

  ```rust
  pub fn new() -> Self {
      Self::default()
  }

  pub fn with_freq(freq: u32) -> Self {
      Self { freq, ..Self::default() }
  }
  ```

#### Error handling

- `thiserror::Error` for all error enums.
- Error variants use `#[error(transparent)]` for wrapping foreign errors;
  `#[from]` for auto-conversions.
- `#[non_exhaustive]` on public error enums to allow future variants.
- The `Result` type alias pattern is used consistently: `type Result<T, E =
  Error> = std::result::Result<T, E>;`
- Errors are reported to the user through a dedicated display mechanism, not raw
  `eprintln!`.
- Functions propagate errors with `?`; only entry points match on errors to
  report and convert them.

#### Control flow

- Early returns for error/guard cases. Happy path is the main body.
- `if let` and `let ... else` preferred over `match` for single-variant checks.
- `match` preferred over chained `if let` when more than two variants are
  relevant.
- Named loop labels (`'label: loop`) used when breaking from nested loops.

#### Comments in function bodies

- Section headers (noun phrase, no punctuation, followed by blank line):

  ```rust
  // Fetch opcode
  let op = self.fetch(bus)?;

  // Execute
  self.exec(op, bus)
  ```

- Inline `// NOTE:` for important caveats about non-obvious behavior.
- Multi-line explanatory comments precede the block they explain, separated by a
  blank line from the next block.

#### Derives & attributes

Each `#[derive]` is scoped to one crate. Std traits come first in their own
`#[derive]`; each third-party crate gets a separate `#[derive]`, ordered
alphabetically by crate name, with traits within each sorted alphabetically.

```rust
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(thiserror::Error)]
```

Std trait order:

| # | Trait        |
|---|--------------|
| 1 | `Copy`       |
| 2 | `Clone`      |
| 3 | `Debug`      |
| 4 | `Default`    |
| 5 | `PartialEq`  |
| 6 | `Eq`         |
| 7 | `PartialOrd` |
| 8 | `Ord`        |
| 9 | `Hash`       |

Attributes on an item are stacked in this order:

- `#[derive(...)]`, always first.
- Structural / semantic attrs: `#[non_exhaustive]`, `#[repr(...)]`,
  `#[error(...)]`, `#[cfg_attr(...)]`, etc.
- Lint suppression, always last: `#[expect(...)]`, `#[allow(...)]`,
  `#[rustfmt::skip]`.

#### Unsafe / attributes

- `#[expect(clippy::...)]` is used with an implicit justification via proximity
  to the code that triggers it.
- `#[rustfmt::skip]` is used sparingly for alignment-heavy blocks.

#### Testing

- Unit tests live in the same file, gated with `#[cfg(test)]`, in a
  `mod tests { use super::*; ... }` block.
- Test function names follow `noun_verb_works` or `noun_verb_panics`
  conventions.
- Integration tests live in `tests/`, one file per test suite.
- Test macros (`macro_rules! test!`) are used to generate repetitive test cases
  from path lists.

---

### Swift

#### Formatting

- 4-space indentation; line length 100.
- Formatted with `swift-format`.
- `@Observable` for observable model classes; `final class` for reference
  types that shouldn't be subclassed.

#### File header

Every Swift file begins with:

```swift
//
//  <FILE>.swift
//  <PROJECT>
//
//  Created by <AUTHOR> on <YYYY-MM-DD>.
//
```

#### Naming

- Types: `UpperCamelCase`.
- Properties and methods: `lowerCamelCase`.
- Global constants: `UPPER_SNAKE_CASE` for numeric constants (`CLOCK`, `BATCH`,
  `AUDIO`).

#### Structure

- Extensions group functionality by protocol conformance or logical category,
  placed after the primary type definition.
- Nested types (`enum`, `class`) are defined inline within the type that owns
  them.
- `private` and `private(set)` access control are used aggressively; `public`
  only at framework boundaries.

#### Documentation

- Doc comments use `///` with complete sentences for methods, fragment
  descriptions for properties.
- `# Note` sections in doc comments for important caveats.

#### Concurrency

- `Thread` used for the emulation loop rather than `async/await` or
  `DispatchQueue`.
- Shared mutable state is protected with `Mutex` (`Synchronization`) or
  `@Atomic` property wrappers.
- `withLock` / `withLockIfAvailable` used for mutex access.
