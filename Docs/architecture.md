# ZapNano Architecture

ZapNano is a terminal text editor written in Rust. It uses ratatui for rendering, crossterm for terminal input and raw mode, ropey as the underlying text buffer data structure, and mlua (Lua 5.4 embedded) for the extension system. Configuration is stored as TOML files on disk.

The binary is built with `cargo build`. On Windows, `build.rs` uses `winres` to embed an application icon into the executable. On other targets the build script does nothing.


## Module Layout

```
src/
  main.rs
  app.rs
  config.rs
  settings.rs
  editor/
    mod.rs
    core.rs
    buffer.rs
    cursor.rs
    highlight.rs
    actions.rs
    history.rs
  input/
    handler.rs
    editor_keys.rs
    keybindings.rs
  ui/
    app_view.rs
    editor_view.rs
    status_bar.rs
    welcome.rs
    explorer.rs
    commands_view.rs
    settings_view.rs
    extensions_view.rs
  plugins/
    mod.rs
    api.rs
    engine.rs
extensions/
  css_pack/init.lua
  html_pack/init.lua
  js_pack/init.lua
  php_pack/init.lua

```


## Entry Point

`main.rs` parses command-line arguments. If `--version` or `-v` is passed it prints the version and exits. If the first argument is `new` followed by a name it scaffolds a new extension directory and exits. Otherwise it loads settings, constructs an `App` with an optional file path, and calls `app.run()`.


## App State

`App` in `app.rs` is the root struct. It owns:

`editors: Vec<Editor>` — every open tab is an `Editor`. `active_editor: usize` tracks which one is currently displayed. Switching tabs changes this index; the editors themselves stay alive.

`prompt: PromptType` — an enum with variants `None`, `Command`, `SaveAs`, and `Search`. When the prompt is active the bottom bar shows a text input instead of the status bar. The string being typed is stored in `prompt_input`.

`command_history: Vec<String>` — completed command strings, navigable with Up/Down while the command prompt is open.

`show_welcome`, `show_settings`, `show_commands`, `show_extensions` — boolean flags that control which overlay is rendered on top of the editor.

`explorer: Explorer` — the file system panel. Has its own `visible` flag.

`lua_engine: Option<LuaEngine>` — the plugin runtime. It is `None` if plugins failed to initialise.

`autocomplete: AutocompleteState` — tracks whether the snippet popup is visible, which options match the current prefix, and which item is selected.

`settings: Settings` — the current configuration. Loaded from disk at startup and saved immediately whenever something changes.

`scroll_offset: usize` — the line index of the topmost visible row. Updated every frame to keep the cursor on screen.

`quit_context: QuitContext` — when quitting with unsaved changes the app does not quit immediately. Instead it sets this to `QuitAll` or `CloseTab(idx)` and waits for the user to confirm with `y`.


## Main Loop

`app.run()` enables raw mode, enters the alternate screen, creates the ratatui terminal, and starts the event loop.

Each iteration:

1. The scroll offset is adjusted so the cursor stays within the visible area.
2. `terminal.draw` is called, which calls `ui::app_view::render`.
3. `event::read()` blocks until a key is received.
4. If a quit confirmation is pending, only `y` or `Y` is handled. `y` completes the quit or tab close; anything else cancels.
5. Otherwise `handle_event` is called with the key.
6. If `should_quit` is true the loop exits.

After the loop, raw mode is disabled and the alternate screen is left.


## Editor

`Editor` in `editor/core.rs` owns a `Buffer`, a `Cursor`, a `Highlighter`, an undo stack, and a redo stack.

The undo stack stores `(Rope, Cursor, bool)` tuples — a snapshot of the full text, cursor position, and modified flag at each editing step. Undo restores the previous snapshot and pushes the current one onto the redo stack. Redo is the reverse. `save_history` is called at the start of every mutating action.


## Buffer

`Buffer` in `editor/buffer.rs` wraps a `ropey::Rope`. All text content lives in the rope. The buffer knows its file path (if any) and a `modified` flag.

Reading a file: `Rope::from_reader(BufReader::new(File::open(path)))`.

Saving: `self.text.write_to(BufWriter::new(File::create(path)))`.

Character insertion, deletion, and string insertion are thin wrappers over rope methods that also set `modified = true`.

`line_to_char(line_idx)` returns the absolute character offset of the start of a line. This is the main bridge between line-based cursor coordinates and the flat character index the rope uses.


## Cursor

`Cursor` in `editor/cursor.rs` tracks position as `(line_idx, char_idx)`. `visual_x` is the column the cursor wants to be in; it is preserved across vertical moves so moving up or down keeps the cursor at the same visual column when possible.

`get_absolute_char_idx` converts the line/column pair to a flat character index via `buffer.line_to_char(line_idx) + char_idx`.

`selection_start: Option<(usize, usize)>` stores the anchor of the selection. When it is `Some`, the region between it and the current cursor is selected. `get_selection_char_bounds` returns the `(start, end)` pair in absolute character indices, always in order regardless of which end the cursor is on.

`snap_to_bounds` clamps the cursor to valid positions after operations that might leave it out of range.


## Highlighting

`Highlighter` in `editor/highlight.rs` combines a built-in tree-sitter pipeline with regex rules from plugins.

The file extension is determined when the highlighter is created. For Rust files, tree-sitter-rust is used. For other extensions the highlighter falls back to regex.

After every edit, `highlighter.update(text)` is called. It re-runs whichever pipeline applies and produces a list of styled spans that `editor_view` consumes when rendering.

Plugin highlight rules are stored in a `PluginRegistry` behind an `Arc<Mutex<>>`. The highlighter holds a reference to the same arc. When plugins register new rules (or are reloaded), the highlighter picks them up automatically because it reads from the shared registry on each update.


## Rendering

`ui/app_view.rs` is the top-level render function. It splits the terminal into three rows: a one-line tab bar at the top, the editor area, and a one-line bottom bar.

The tab bar iterates over all editors and renders each tab title. The active tab uses theme colors for emphasis and a `*` suffix for modified files.

The bottom bar shows either the command/search/save-as prompt or the status bar. The status bar shows the file path and the current cursor position as `line:column`.

Overlays — welcome screen, settings, extensions, explorer, command reference, quit confirmation, autocomplete popup — are rendered on top of the editor using `f.render_widget(Clear, area)` followed by the overlay widget. Only one overlay is active at a time; the order in `app_view.rs` determines priority.

The autocomplete popup appears at the cursor position. If there is not enough room below the cursor it appears above instead.


## Input Routing

`input/handler.rs` is the dispatch layer. It checks which overlay is active and routes the key event to the appropriate handler:

Settings overlay: Up/Down navigate fields, Left/Right change values, Escape saves and closes.

Extensions overlay: Up/Down navigate the list, Enter toggles a plugin on or off, `r`/`R` reloads all plugins without restarting.

Commands overlay: Escape or Enter closes it.

Welcome screen: Up/Down navigate, Enter selects an action.

Explorer: Up/Down navigate, Enter opens a file or expands a directory, Escape returns to the editor or welcome screen.

Prompt (command, save-as, search): character input appends to `prompt_input`, Backspace removes the last character, Escape cancels, Enter submits.

If none of these are active, the event goes to `editor_keys.rs`.

`editor_keys.rs` maps key events to `Command` enum variants via `keybindings::map_key_event` and then acts on the command. After handling movement commands it also runs the autocomplete logic: it reads back from the cursor to find the current word prefix, filters the snippet list for that extension, and updates `app.autocomplete`.


## Command System

The command prompt is opened by typing `!. ` (exclamation mark, period, space) anywhere in the editor. The editor keys handler detects this three-character sequence, undoes all three inserted characters, clears the redo stack, and opens the prompt.

Supported commands:

`open <path>` — opens a file in a new tab.

`new` — creates an empty tab.

`close` — closes the active tab, with an unsaved-changes confirmation if needed.

`rename <path>` — renames the current file on disk and updates the buffer path.

Empty input or unknown commands close the prompt without doing anything.

Command strings are appended to `command_history` and navigable with Up/Down while the prompt is open.


## Search

Search is triggered by the configured keybinding. The query is typed into the prompt. As characters are added or removed, `app.update_search()` recomputes `search_results` — a list of absolute character offsets where the query appears in the current buffer. Each Enter press cycles to the next match by advancing `search_idx` and moving the cursor to that offset.


## Plugin System

Described in detail in `extensions.md`.


## Settings

Described in detail in `reference.md`.


## Theme System

Described in detail in `themes-and-extensions.md`.
