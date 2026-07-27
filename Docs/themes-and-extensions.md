# Extensions and Themes


## Extensions

Extensions are Lua scripts that run inside the editor process. They can register syntax highlighting rules and code snippets for any file type.


### Directory Location

ZapNano looks for extensions in one of two places, checked in order:

1. A `resources/extensions` directory relative to the parent of the directory containing the executable. This is where packaged installations put bundled extensions.
2. An `extensions` directory in the current working directory. This is the fallback used during development and when running from source.

Each extension is a subdirectory of the extensions directory. The subdirectory name is the extension's identifier. The entry point is a file named `init.lua` inside that directory.


### Extension Structure

An extension's `init.lua` must return a Lua table with the following fields:

```lua
return {
    name    = "my_extension",
    author  = "Your Name",
    version = "1.0.0",

    on_load = function()
        -- called once when the extension is loaded
    end
}
```

All three string fields (`name`, `author`, `version`) are required. If any are missing the extension will fail to load and an error will be written to `zapnano.log`.

The `on_load` function is called once during startup, or immediately when extensions are reloaded via the Extensions overlay. All registration calls should happen inside `on_load`.


### The zap API

Inside `on_load` you have access to a global table called `zap`. It provides two registration functions and one utility.

**zap.print(message)**

Appends a line to `zapnano.log` in the current working directory. Useful for debugging.

```lua
zap.print("my_extension loaded")
```

**zap.register_highlighter(extension, rules)**

Registers regex-based syntax highlighting rules for files with the given extension.

`extension` is a string like `"js"`, `"css"`, `"lua"`. It must match the file extension without the leading dot.

`rules` is an array of tables. Each table has two keys:

`pattern` — a regular expression string. Rust regex syntax is used.

`color` — a color name string. Accepted values are listed below.

Rules are applied in the order they appear. Earlier rules take priority over later ones for the same characters.

```lua
zap.register_highlighter("lua", {
    { pattern = "--[^\n]*",      color = "DarkGray" },
    { pattern = "\\b(function|end|if|then|else|return|local)\\b", color = "Magenta" },
    { pattern = "['\"][^'\"]*['\"]", color = "Yellow" },
    { pattern = "\\b\\d+\\b",   color = "LightBlue" },
})
```

**zap.register_snippets(extension, rules)**

Registers code snippets for files with the given extension.

`extension` is the same format as for `register_highlighter`.

`rules` is an array of tables. Each table has three keys:

`prefix` — the text the user types to trigger this snippet. Must be at least two characters for the autocomplete popup to appear.

`body` — the text that replaces the prefix when the snippet is inserted. Use `\n` for newlines. A `|` character in the body marks the cursor position after insertion; it is removed from the inserted text.

`description` — a short label shown in the autocomplete popup.

```lua
zap.register_snippets("lua", {
    { prefix = "fn",  body = "function @1(@2)\n    |\nend", description = "Function" },
    { prefix = "loc", body = "local @1 = @2",               description = "Local variable" },
})
```

The `@1`, `@2` placeholders are inserted as literal text. They are not interactive tab stops.


### Accepted Color Names

These names are accepted by `register_highlighter`. Names are case-insensitive.

```
Black
Red
Green
Yellow
Blue
Magenta
Cyan
DarkGray
LightRed
LightGreen
LightYellow
LightBlue
LightMagenta
LightCyan
White
```

Some built-in extensions also use the names `Keyword`, `Function`, `String`, `Variable`, `Operator`, `Special`. These map to specific terminal colors determined by the highlight module; they behave identically to the named colors above.


### Scaffolding a New Extension

```
zapnano new my_extension
```

This creates `extensions/my_extension/init.lua` with a minimal template and exits. You can then edit the file and reload extensions from inside the editor without restarting.


### Reloading Extensions

Open the Extensions overlay from the welcome screen. The list shows all installed extensions with an indicator for whether each one is enabled. Press `r` or `R` to reload all enabled extensions immediately. This re-runs every `on_load` function and updates all open buffers.


### Enabling and Disabling Extensions

In the Extensions overlay, navigate to an extension and press Enter to toggle it. Disabled extensions are recorded in `settings.toml` under `disabled_extensions`. The change takes effect on the next reload or restart.


### Multiple Calls to register_highlighter

Calling `register_highlighter` for the same extension more than once appends to the existing rule list; it does not replace it. This means multiple extensions can each add rules for the same file type and all rules will apply.


## Built-in Extensions

ZapNano ships with five extensions. They are loaded the same way as user extensions.

**css_pack** handles `.css` files. Highlights at-rules, selectors, properties, values, units, comments, and url() references. Provides snippets for common patterns including flexbox, grid, media queries, animations, and positioning.

**html_pack** handles `.html` files. Highlights tags, attributes, strings, comments, entities, and also applies inline JavaScript and CSS property highlighting within the same file. Provides snippets for the HTML5 boilerplate, common elements, form controls, and link/script tags.

**js_pack** handles `.js` and `.ts` files. For JavaScript it highlights keywords, built-in globals, literals, strings including template literals, comments, and function calls. For TypeScript it also highlights type-specific keywords. Provides JavaScript snippets for console.log, arrow functions, loops, promises, and imports. Provides TypeScript snippets for interfaces and type aliases.

**php_pack** handles `.php` files. Highlights PHP tags, keywords, built-in functions, variables (including the `$` sigil), operators, strings, and comments.



## Themes

Themes control the colors used for the interface chrome: tab bar, status bar, prompt, selection, and line numbers. They do not directly control syntax highlighting colors; those are determined by the regex patterns in extension highlight rules.

A theme is a TOML file stored in the themes directory:

```
Linux / macOS    ~/.config/zapnano/themes/
Windows          %APPDATA%\zapnano\themes\
```

The file name without the extension is the theme name used in settings.


### Theme File Format

```toml
bg             = "#1e1e1e"
fg             = "#d4d4d4"
tab_bar_bg     = "#252526"
tab_active_bg  = "#1e1e1e"
tab_active_fg  = "#ffffff"
tab_inactive_fg = "#858585"
prompt_bg      = "#007acc"
prompt_fg      = "#ffffff"
status_bg      = "#007acc"
status_fg      = "#ffffff"
selection_bg   = "#264f78"
line_nr_fg     = "#858585"
```

All fields are optional. Missing fields fall back to hardcoded defaults listed below. Values can be hex color codes (`#rrggbb`) or named colors (`black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `gray`, `darkgray`, `lightred`, `lightgreen`, `lightyellow`, `lightblue`, `lightmagenta`, `lightcyan`, `white`). Named colors resolve to the terminal's palette colors, not fixed RGB values.

Default values when a field is missing:

```
bg              terminal default (transparent)
fg              terminal default (transparent)
tab_bar_bg      black
tab_active_bg   blue
tab_active_fg   white
tab_inactive_fg darkgray
prompt_bg       blue
prompt_fg       white
status_bg       white
status_fg       black
selection_bg    darkgray
line_nr_fg      darkgray
```


### Built-in Themes

These themes are installed automatically into the themes directory on first launch. They can be edited like any other theme file; the editor will not overwrite existing files.

**Default** — no explicit colors, uses terminal defaults throughout.

**Dracula** — dark purple background (`#282a36`) with muted accent colors from the Dracula palette.

**Oceanic** — dark teal background (`#1b2b34`) with blue accents.

**Light** — light grey background (`#fafafa`) with dark text and blue accents.

**Monokai** — dark background (`#272822`) with cyan accents.

**Nord** — dark blue-grey background (`#2e3440`) with light blue accents from the Nord palette.

**Gruvbox** — dark brown background (`#282828`) with warm yellow accents.

**SolarizedDark** — very dark teal background (`#002b36`) with blue accents.


### Creating a Theme

Create a `.toml` file in the themes directory. The file name becomes the theme name. Restart the editor or open Settings to see it in the theme list. Any fields you omit inherit the defaults listed above.

Example — a minimal red accent theme:

```toml
bg            = "#1a1a1a"
fg            = "#e0e0e0"
tab_bar_bg    = "#111111"
tab_active_bg = "#aa2222"
tab_active_fg = "#ffffff"
status_bg     = "#aa2222"
status_fg     = "#ffffff"
```

Save it as `~/.config/zapnano/themes/RedAccent.toml`, then select `RedAccent` in Settings.
