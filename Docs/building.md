# Building ZapNano

## Requirements

Rust toolchain (stable). Install via rustup if not already present.

On Windows, the build script uses `winres` to embed the application icon. `winres` requires a resource compiler. This is included with Visual Studio Build Tools (install the "Desktop development with C++" workload). If the resource compiler is not found, `build.rs` prints a warning and the build continues without embedding the icon.

No other system dependencies are required. All C libraries including Lua 5.4 are vendored as Rust crates and compiled from source by Cargo.


## Building

```
cargo build          # debug build
cargo build --release
```

The release binary is at `target/release/zapnano` on Linux and macOS, `target/release/zapnano.exe` on Windows.


## Running from Source

```
cargo run                      # welcome screen
cargo run -- path/to/file      # open a file
cargo run -- new my_extension  # scaffold an extension
```

When running from source, the extensions directory is resolved relative to the compiled binary inside `target/`. The `get_extensions_dir()` function will not find the `resources/extensions` path there, so it falls back to the `extensions/` directory in the working directory where you run the command. Run from the project root so the bundled extensions are found.


## Project Layout

```
ZapNano/
  Cargo.toml
  build.rs
  src/
    main.rs
    app.rs
    config.rs
    settings.rs
    editor/
    input/
    ui/
    plugins/
  extensions/
    css_pack/
    html_pack/
    js_pack/
    php_pack/

  installerWindows/
    resources/
      images/
        ZapNano.ico
```

Source code lives in `src/`. Extension scripts live in `extensions/`. The installer assets for Windows are in `installerWindows/`.


## Dependencies

```toml
ratatui = "0.26.3"          # TUI framework
crossterm = "0.28.1"        # terminal backend and input
ropey = "1.6.1"             # rope data structure for text buffers
tree-sitter = "0.22.6"      # incremental parsing
tree-sitter-highlight = "0.22.6"
tree-sitter-rust = "0.21.2" # Rust grammar for tree-sitter
anyhow = "1.0.86"           # error handling
mlua = { version = "0.9.9", features = ["lua54", "vendored"] }  # Lua 5.4 embedded
regex = "1.10.5"            # regex engine for plugin highlight rules
serde = { version = "1.0.228", features = ["derive"] }
toml = "1.1.2"              # config file parsing
dirs = "6.0.0"              # platform config directory resolution
```

The `vendored` feature on `mlua` means Lua 5.4 is compiled from C source included in the crate. No system Lua installation is needed.


## Code Organisation Notes

The `Editor` struct is split across several files in `src/editor/`. `core.rs` defines the struct. `actions.rs` adds editing methods (`insert_char`, `delete_backwards`, etc.). `history.rs` adds `undo` and `redo`. `mod.rs` re-exports `Editor` from `core`. This keeps each concern in a separate file while presenting a unified type to the rest of the codebase.

The `App` struct in `app.rs` owns the event loop. The input handlers in `src/input/` take `&mut App` and mutate it directly. There is no message-passing or command queue; all state changes happen synchronously in the event handling path.

The `Settings` struct uses `serde` derive macros for serialisation and deserialisation. The `#[serde(default)]` attribute on fields means settings files that predate the addition of a field continue to load correctly with the field set to its default.

`ThemeColors` is reconstructed from disk on every render frame by calling `ThemeColors::get(theme_name)`. This is intentional: it means theme files can be edited and the changes are visible on the next frame without restarting the editor. The cost is acceptable because file reads on a local theme directory are fast, but it could be cached if profiling showed it to be a problem.


## Adding a New Command

1. Add a match arm in the command dispatch block in `src/input/handler.rs`, inside the `PromptType::Command` branch.
2. Implement the logic there directly or as a method on `App`.
3. Document the command in `reference.md`.


## Adding a New Keybinding

1. Add a variant to the `Command` enum in `src/input/keybindings.rs`.
2. Add a match arm in `map_key_event` mapping the key combination to the new variant.
3. Handle the variant in `src/input/editor_keys.rs`.


## Adding Support for a New Tree-sitter Grammar

1. Add the grammar crate to `Cargo.toml`.
2. In `src/editor/highlight.rs`, extend the extension-to-language match to include the new file extension.
3. Initialise the highlight configuration for the new language the same way Rust is currently initialised.


## Packaging

On Windows, `build.rs` embeds the icon from `installerWindows/resources/images/ZapNano.ico` using `winres`. An installer can then be built with a tool such as NSIS or Inno Setup pointing at the compiled binary and the `extensions/` and `themes/` directories.

On Linux and macOS, the binary is self-contained. Extensions should be distributed alongside the binary in a `resources/extensions/` directory one level above the directory containing the binary, matching the path checked by `get_extensions_dir()`.
