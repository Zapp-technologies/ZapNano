# Plugin System Internals

This document covers how the Lua extension system is implemented in Rust. It is intended for contributors working on the editor core, not for extension authors. Extension authors should read `themes-and-extensions.md`.


## Overview

The plugin system has three parts:

`src/plugins/api.rs` defines the data types that extensions can populate and exposes the Lua API surface.

`src/plugins/engine.rs` owns the Lua runtime and handles loading extension scripts from disk.

`src/editor/highlight.rs` consumes the populated registry when rendering text.


## PluginRegistry

`PluginRegistry` is a plain struct with two `HashMap` fields:

```rust
pub struct PluginRegistry {
    pub highlighters: HashMap<String, Vec<HighlightRule>>,
    pub snippets: HashMap<String, Vec<SnippetRule>>,
}
```

Both maps are keyed by file extension string (e.g. `"js"`, `"css"`).

`HighlightRule` holds a pattern string and a color name string. The color name is resolved to a `ratatui::style::Color` at render time by the highlighter, not at registration time.

`SnippetRule` holds a prefix, a body, and a description.

The registry is wrapped in `Arc<Mutex<PluginRegistry>>` so it can be shared between the Lua engine (which writes to it on extension load or reload) and all open editor instances (which read from it on every highlight update).


## Lua API Registration

`register_api(lua, registry)` in `api.rs` is called once when the `LuaEngine` is created. It creates a Lua table called `zap` and sets it as a global.

Three functions are attached to this table:

`zap.print` — creates a Lua function that receives a string and appends it to `zapnano.log`. Uses `OpenOptions::new().create(true).append(true)` so the file is created if absent and never truncated.

`zap.register_highlighter` — receives a file extension string and an array of tables. Each table must have `pattern` and `color` keys. Missing keys are silently skipped. Valid rules are pushed into `registry.highlighters[extension]`. Registering the same extension multiple times appends to the existing list.

`zap.register_snippets` — receives a file extension string and an array of tables. Each table must have `prefix` and `body` keys; `description` is optional and defaults to an empty string. Valid rules are pushed into `registry.snippets[extension]`.

Both registration functions capture a clone of the `Arc<Mutex<PluginRegistry>>` rather than borrowing it. This is necessary because Lua closures require `'static` lifetimes.


## LuaEngine

`LuaEngine` in `engine.rs` owns the `Lua` instance and a reference to the shared `PluginRegistry`.

```rust
pub struct LuaEngine {
    pub lua: Lua,
    pub registry: Arc<Mutex<PluginRegistry>>,
}
```

Construction: `LuaEngine::new(disabled_extensions)` creates a new `Lua` instance, creates the registry, calls `register_api`, then calls `load_all_plugins`.

`get_extensions_dir()` resolves the extensions directory. It walks up from the executable path looking for `../resources/extensions`. If that path does not exist (typical during development) it falls back to `./extensions`. This means the lookup is relative to the executable, not the working directory, in packaged builds.

`load_all_plugins(ext_dir, disabled)` reads the directory, skips any entry whose name is in `disabled`, and calls `load_plugin` on `init.lua` files found inside subdirectories.

`load_plugin(path)` reads the file as a string, evaluates it as a Lua chunk, and expects the return value to be a table. It then reads `name`, `author`, and `version` fields from that table (returning an error if any are missing), calls `on_load()`, and logs success. If `on_load` returns a Lua error it is caught and logged; it does not crash the editor.

`load_script(script)` executes an arbitrary Lua string. It is not currently called from production code but is available for future use.


## Reload Flow

When the user presses `r` or `R` in the Extensions overlay, the handler does:

```rust
app.lua_engine = LuaEngine::new(&app.settings.disabled_extensions).ok();
if let Some(engine) = &app.lua_engine {
    for editor in &mut app.editors {
        editor.highlighter.plugin_registry = Some(Arc::clone(&engine.registry));
        let text = editor.buffer.text.to_string();
        editor.highlighter.update(&text);
    }
}
```

A completely new `Lua` instance is created, which means all previously registered rules are discarded. The new registry is then distributed to every open editor's highlighter and each highlighter is updated so the new rules take effect immediately.


## Highlighter Integration

`Highlighter` in `editor/highlight.rs` holds an `Option<Arc<Mutex<PluginRegistry>>>`. When the file extension matches a tree-sitter grammar (currently only `rs` for Rust), tree-sitter handles highlighting. For all other extensions the highlighter reads the matching rule list from the registry.

The highlight pass produces a `Vec` of `(char_range, Color)` pairs. The editor view iterates over the text line by line and applies these spans when building the ratatui `Text` widget.

If the registry lock cannot be acquired, highlighting for that frame is skipped and the text is rendered without color. This prevents a deadlock from blocking the draw loop.


## Autocomplete Integration

The autocomplete logic lives in `editor_keys.rs` and runs after every key event. It does not use the registry directly through a function call; it borrows the same arc from `app.lua_engine`:

```rust
if let Some(registry_arc) = app.lua_engine.as_ref().map(|e| Arc::clone(&e.registry)) {
    if let Ok(registry) = registry_arc.lock() {
        // read registry.snippets[extension]
    }
}
```

It reads the current line up to the cursor, extracts the word prefix by scanning backwards over alphanumeric characters and `@` and `_`, and filters the snippet list for entries whose prefix starts with that word. If at least two characters have been typed and at least one snippet matches, the autocomplete popup is shown.

Tab expansion calls `editor.insert_snippet(body, prefix_len)`. This method first deletes `prefix_len` characters to the left of the cursor (removing the typed prefix), then inserts each character of the body one by one. When it encounters a `|` character it records its position without inserting it. After inserting the full body it moves the cursor left by the number of characters that followed the `|`, placing the cursor at the marked position.


## Error Handling

Plugin load errors are written to `zapnano.log` via the `zap.print` function (which the engine calls directly by looking up the function in globals). The editor continues running with whatever extensions loaded successfully. There is no user-visible error display for extension failures.

If `LuaEngine::new` itself fails (for example if the Lua runtime cannot be initialised), `app.lua_engine` is `None` and the editor runs without any extensions. No highlighting or autocomplete is provided in this case beyond the built-in tree-sitter pass for Rust files.
