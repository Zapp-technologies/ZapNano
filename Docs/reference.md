# ZapNano Reference

ZapNano is a terminal text editor. It runs in the terminal and uses the alternate screen buffer, so it does not leave output in your shell history when it exits.


## Starting the Editor

```
zapnano                    # opens the welcome screen
zapnano path/to/file       # opens a file directly
zapnano path/to/directory  # opens the file explorer rooted at that directory
zapnano --version          # prints the version
zapnano new <name>         # scaffolds a new extension (see extensions.md)
```


## Welcome Screen

When opened without a file argument the welcome screen is shown. It lists recent files (up to 8) and a set of actions. Use Up and Down to navigate, Enter to select.

Actions available from the welcome screen: New File, Explore Workspace, Quit Editor, Command Reference, Settings, Extensions.


## Tabs

Each open file is a tab. The tab bar at the top shows numbered tabs. The active tab is highlighted. Tabs marked with `*` have unsaved changes.

Switching tabs: the keybinding for previous and next tab cycles through all open tabs. Tabs wrap around at both ends.

Opening a file adds a new tab. Closing a tab removes it. If a tab has unsaved changes you will be asked to confirm before it is closed.


## Editing

Standard character input inserts at the cursor position. The following behaviours are automatic:

Bracket and quote auto-closing: typing `(`, `{`, `[`, or `"` inserts the matching closing character immediately after and leaves the cursor between them.

Tab inserts spaces, not a tab character. The number of spaces is controlled by the Tab Size setting.


## Keyboard Shortcuts

The following are the defaults. They are defined in `src/input/keybindings.rs`.

```
Ctrl+S          Save
Ctrl+Q          Quit
Ctrl+F          Open search prompt
Ctrl+Z          Undo
Ctrl+Y          Redo
Ctrl+A          Select all
Delete          Delete character forward
Backspace       Delete character backward
Arrow keys      Move cursor
Shift+Arrow     Extend selection
Ctrl+Left       Previous tab
Ctrl+Right      Next tab
Escape          Return to welcome screen (if file is new and unmodified)
```

Typing `!. ` (exclamation mark, period, space) anywhere in the editor opens the command prompt. The three characters are removed from the buffer before the prompt opens.


## Commands

Commands are entered in the command prompt, which is opened by typing `!. `.

```
open <path>        Open a file in a new tab
new                Create an empty tab
close              Close the active tab
rename <path>      Rename the current file and save it under the new path
```

Pressing Escape cancels the prompt without running anything. Previous commands can be recalled with Up and Down while the prompt is open.


## Search

Press the search keybinding to open the search prompt. Type to filter. Results are highlighted as you type. Press Enter to jump to the next match. The search wraps around. Press Escape to close the prompt.


## File Explorer

The explorer shows a tree of the file system starting from the directory ZapNano was opened in, or from whichever directory was passed as an argument. Up and Down navigate the list. Enter opens a file or expands a directory. Escape closes the explorer.

If the current tab is an empty unsaved file when the explorer is closed, the welcome screen is shown instead of returning to the editor.


## Settings

Open Settings from the welcome screen or via the command reference.

Settings are stored at:

```
Linux / macOS    ~/.config/zapnano/settings.toml
Windows          %APPDATA%\zapnano\settings.toml
```

The file is written automatically whenever a setting changes. You can also edit it by hand; changes take effect on the next launch.

Available settings:

```toml
tab_size = 4
theme = "Default"
disable_autocomplete = false
disabled_extensions = []
recent_files = []
```

`tab_size` accepts integers from 1 to 16.

`theme` must match the filename of a `.toml` file in the themes directory (without the extension). Built-in themes are listed in `themes-and-extensions.md`.

`disable_autocomplete` suppresses the snippet popup entirely when true.

`disabled_extensions` is a list of extension directory names that should not be loaded.

`recent_files` is managed automatically. It holds up to 8 absolute paths of recently opened files, most recent first.

Inside the Settings overlay, Up and Down select a field. Left and Right change its value. Escape saves and closes.


## Autocomplete

When you type two or more characters that match the prefix of a registered snippet for the current file type, a popup appears near the cursor listing matching snippets. Tab selects the highlighted entry and expands it. Up and Down navigate the list. Escape dismisses the popup.

Snippets are provided by extensions. The built-in extensions cover CSS, HTML, JavaScript, TypeScript, and PHP. See `themes-and-extensions.md` for the snippet lists.

The `@1`, `@2`, etc. placeholders in snippet bodies are literal text in the current implementation; they are not interactive tab stops.


## Status Bar

The bottom line of the screen shows the current file path and the cursor position as `line:column`. Both numbers are 1-based. The path shows `[No Name]` for unsaved buffers. A `[+]` suffix appears when the buffer has unsaved changes.


## Unsaved Changes

If you try to quit or close a tab with unsaved changes, a confirmation message appears. Press `y` or `Y` to proceed. Any other key cancels.


## Logging

Plugins can write to a log file by calling `zap.print(message)`. Output goes to `zapnano.log` in the current working directory, appended. The log file is created if it does not exist. The main application does not write to this file.
