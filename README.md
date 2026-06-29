# ZapNano

A terminal text editor written in Rust.

```
  ______               _   _
 |__  /               | \ | |
    / /  __ _  _ __   |  \| |  __ _  _ __    ___
   / /  / _` || '_ \  | . ` | / _` || '_ \  / _ \
  / /__| (_| || |_) | | |\  || (_| || | | || (_) |
 /_____|\__,_|| .__/  |_| \_| \__,_||_| |_| \___/
              | |
              |_|
```


## What it is

ZapNano runs in the terminal. It supports multiple tabs, undo and redo, text selection, search, a file explorer, and a Lua-based extension system for syntax highlighting and code snippets. Themes are TOML files that control the interface colors.


## Building

Requires a stable Rust toolchain.

```
cargo build --release
```

On Windows the build script embeds an application icon using `winres`, which requires the Visual Studio C++ build tools. If those are not present the build still succeeds, just without the icon.

The release binary ends up at `target/release/zapnano` on Linux and macOS, `target/release/zapnano.exe` on Windows.

## Installation

Pre-packaged installers are available for both Windows and Linux on the [Releases page](https://github.com/Zapp-technologies/ZapNano/releases). They install `zapnano` directly to your local user environment, along with a `znano` alias script and bundled extensions.

**Windows:**
Compile the `installerWindows/zapnano.iss` script using the Inno Setup Compiler (`ISCC.exe`) to generate `zapnano_setup.exe`. Run the setup executable to install ZapNano on your system without requiring administrator privileges.

**Linux:**
Run the provided package script `installerLinux/build_tarball.sh` to generate a distributable `tar.gz` archive. When extracted, simply run the included `install.sh` script to securely install the binary and resources into `~/.local/share/ZapNano` and automatically wire up symlinks to `~/.local/bin`.


## Usage

```
zapnano                    open the welcome screen
zapnano path/to/file       open a file
zapnano path/to/directory  open the file explorer at that path
zapnano --version
zapnano new <name>         scaffold a new extension
```


## Keyboard shortcuts

```
Ctrl+S          save
Ctrl+Q          quit
Ctrl+F          search
Ctrl+Z          undo
Ctrl+Y          redo
Ctrl+A          select all
Ctrl+Left       previous tab
Ctrl+Right      next tab
Shift+Arrow     extend selection
Escape          return to welcome screen (empty unsaved buffer only)
```

Typing `!. ` (exclamation mark, period, space) anywhere in the editor opens the command prompt. The three characters are removed before the prompt opens.


## Commands

```
open <path>     open a file in a new tab
new             create an empty tab
close           close the active tab
rename <path>   rename the current file on disk
```


## Extensions

Extensions are Lua scripts. Each extension is a directory inside the `extensions/` folder containing an `init.lua` file.

```lua
return {
    name    = "my_extension",
    author  = "Your Name",
    version = "1.0.0",

    on_load = function()
        zap.register_highlighter("ext", {
            { pattern = "\\b(keyword)\\b", color = "Magenta" },
        })

        zap.register_snippets("ext", {
            { prefix = "fn", body = "function @1(@2)\n    |\nend", description = "Function" },
        })
    end
}
```

The `|` in a snippet body marks where the cursor lands after expansion. Extensions can be toggled and reloaded without restarting from the Extensions screen.

Four extensions are bundled: `css_pack`, `html_pack`, `js_pack` (also covers TypeScript), and `php_pack`.


## Themes

Themes are TOML files in the platform config directory.

```
Linux / macOS    ~/.config/zapnano/themes/
Windows          %APPDATA%\zapnano\themes\
```

```toml
bg             = "#1e1e1e"
fg             = "#d4d4d4"
tab_bar_bg     = "#252526"
tab_active_bg  = "#007acc"
tab_active_fg  = "#ffffff"
tab_inactive_fg = "#858585"
prompt_bg      = "#007acc"
prompt_fg      = "#ffffff"
status_bg      = "#007acc"
status_fg      = "#ffffff"
selection_bg   = "#264f78"
line_nr_fg     = "#858585"
```

All fields are optional. Eight themes ship built-in: Default, Dracula, Oceanic, Light, Monokai, Nord, Gruvbox, SolarizedDark. Select a theme in the Settings screen or edit `settings.toml` directly.


## Settings

```
Linux / macOS    ~/.config/zapnano/settings.toml
Windows          %APPDATA%\zapnano\settings.toml
```

```toml
tab_size             = 4
theme                = "Default"
disable_autocomplete = false
disabled_extensions  = []
```


## Documentation

[`architecture.md`](Docs/architecture.md) — how the code is structured and how each subsystem works.

[`reference.md`](Docs/reference.md) — full user reference including all keybindings, commands, and settings.

[`themes-and-extensions.md`](Docs/themes-and-extensions.md) — how to write extensions and themes, the full Lua API, all accepted color names, and the built-in extension snippet lists.

[`plugin-internals.md`](Docs/plugin-internals.md) — implementation details of the Lua runtime integration for contributors.

[`building.md`](Docs/building.md) — build requirements, project layout, dependencies, and notes on extending the editor.

[`Changelogs/`](Docs/Changelogs/) — release notes and project history.
