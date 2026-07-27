use crate::app::App;
use crossterm::event::{Event, KeyCode, KeyEvent};
use anyhow::Result;

pub fn handle_event(app: &mut App, event: Event) -> Result<()> {
    if app.show_settings {
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
            match code {
                KeyCode::Esc => {
                    app.show_settings = false;
                    app.settings.save();
                }
                KeyCode::Up => {
                    app.settings_focus_idx = app.settings_focus_idx.saturating_sub(1);
                }
                KeyCode::Down => {
                    app.settings_focus_idx = app.settings_focus_idx.saturating_add(1).min(2);
                }
                KeyCode::Left => {
                    if app.settings_focus_idx == 0 {
                        app.settings.tab_size = app.settings.tab_size.saturating_sub(1).max(1);
                    } else if app.settings_focus_idx == 1 {
                        let themes = crate::settings::get_available_themes();
                        let current_theme = app.settings.theme.as_deref().unwrap_or("Default");
                        let mut idx = themes.iter().position(|t| t == current_theme).unwrap_or(0);
                        if idx > 0 { idx -= 1; } else { idx = themes.len() - 1; }
                        app.settings.theme = Some(themes[idx].to_string());
                    } else if app.settings_focus_idx == 2 {
                        app.settings.disable_autocomplete = !app.settings.disable_autocomplete;
                    }
                }
                KeyCode::Right => {
                    if app.settings_focus_idx == 0 {
                        app.settings.tab_size = app.settings.tab_size.saturating_add(1).min(16);
                    } else if app.settings_focus_idx == 1 {
                        let themes = crate::settings::get_available_themes();
                        let current_theme = app.settings.theme.as_deref().unwrap_or("Default");
                        let mut idx = themes.iter().position(|t| t == current_theme).unwrap_or(0);
                        if idx + 1 < themes.len() { idx += 1; } else { idx = 0; }
                        app.settings.theme = Some(themes[idx].to_string());
                    } else if app.settings_focus_idx == 2 {
                        app.settings.disable_autocomplete = !app.settings.disable_autocomplete;
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    }

    if app.show_extensions {
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
            match code {
                KeyCode::Esc => {
                    app.show_extensions = false;
                    app.settings.save();
                }
                KeyCode::Up => {
                    app.extensions_focus_idx = app.extensions_focus_idx.saturating_sub(1);
                }
                KeyCode::Down => {
                    let mut exts = Vec::new();
                    let ext_dir = crate::plugins::engine::get_extensions_dir();
                    if let Ok(entries) = std::fs::read_dir(&ext_dir) {
                        for entry in entries.flatten() {
                            if entry.path().is_dir() {
                                exts.push(());
                            }
                        }
                    }
                    if !exts.is_empty() && app.extensions_focus_idx + 1 < exts.len() {
                        app.extensions_focus_idx += 1;
                    }
                }
                KeyCode::Enter => {
                    let mut exts = Vec::new();
                    let ext_dir = crate::plugins::engine::get_extensions_dir();
                    if let Ok(entries) = std::fs::read_dir(&ext_dir) {
                        for entry in entries.flatten() {
                            if entry.path().is_dir() {
                                if let Some(n) = entry.file_name().to_str() {
                                    exts.push(n.to_string());
                                }
                            }
                        }
                    }
                    exts.sort();
                    if app.extensions_focus_idx < exts.len() {
                        let target = &exts[app.extensions_focus_idx];
                        if let Some(pos) = app.settings.disabled_extensions.iter().position(|x| x == target) {
                            app.settings.disabled_extensions.remove(pos);
                        } else {
                            app.settings.disabled_extensions.push(target.clone());
                        }
                        app.settings.save();
                    }
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    app.lua_engine = crate::plugins::engine::LuaEngine::new(&app.settings.disabled_extensions).ok();
                    if let Some(engine) = &app.lua_engine {
                        for editor in &mut app.editors {
                            editor.highlighter.plugin_registry = Some(std::sync::Arc::clone(&engine.registry));
                            let text = editor.buffer.text.to_string();
                            editor.highlighter.update(&text);
                        }
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    }

    if app.show_commands {
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
            match code {
                KeyCode::Esc | KeyCode::Enter => {
                    app.show_commands = false;
                }
                _ => {}
            }
        }
        return Ok(());
    }

    if app.show_welcome {
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
            let max_idx = app.settings.recent_files.len() + 6;
            match code {
                KeyCode::Up => {
                    app.welcome_idx = app.welcome_idx.saturating_sub(1).max(1);
                }
                KeyCode::Down => {
                    app.welcome_idx = app.welcome_idx.saturating_add(1).min(max_idx);
                }
                KeyCode::Enter => {
                    let recent_len = app.settings.recent_files.len();
                    if app.welcome_idx <= recent_len {
                        let file = app.settings.recent_files[app.welcome_idx - 1].clone();
                        let registry_opt = app.lua_engine.as_ref().map(|e| std::sync::Arc::clone(&e.registry));
                        if let Ok(mut tmp_editor) = crate::editor::Editor::new(Some(std::path::PathBuf::from(&file)), registry_opt) {
                            let text_contents = tmp_editor.buffer.text.to_string();
                            tmp_editor.highlighter.update(&text_contents);
                            app.editors.push(tmp_editor);
                            app.active_editor = app.editors.len() - 1;
                        }
                        app.show_welcome = false;
                    } else {
                        let action_idx = app.welcome_idx - recent_len;
                        match action_idx {
                            1 => { app.show_welcome = false; }
                            2 => { app.show_welcome = false; app.explorer.visible = true; }
                            3 => { app.should_quit = true; }
                            4 => { app.show_welcome = false; app.show_commands = true; }
                            5 => { app.show_welcome = false; app.show_settings = true; }
                            6 => { app.show_welcome = false; app.show_extensions = true; }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    }

    if app.explorer.visible {
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
            match code {
                KeyCode::Esc => {
                    app.explorer.visible = false;
                    let active = app.active_editor;
                    if app.editors[active].buffer.filepath.is_none() && !app.editors[active].buffer.modified {
                        app.show_welcome = true;
                    }
                }
                KeyCode::Up => {
                    app.explorer.selected_idx = app.explorer.selected_idx.saturating_sub(1);
                }
                KeyCode::Down => {
                    if app.explorer.selected_idx + 1 < app.explorer.items.len() {
                        app.explorer.selected_idx += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(file_path) = app.explorer.toggle_selected() {
                        let active = app.active_editor;
                        if app.editors[active].buffer.modified {
                            app.save().unwrap_or_default();
                        }
                        app.explorer.visible = false;
                        let registry_opt = app.lua_engine.as_ref().map(|e| std::sync::Arc::clone(&e.registry));
                        if let Ok(mut new_editor) = crate::editor::Editor::new(Some(file_path), registry_opt) {
                            let text = new_editor.buffer.text.to_string();
                            new_editor.highlighter.update(&text);
                            app.editors.push(new_editor);
                            app.active_editor = app.editors.len() - 1;
                        }
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    }

    if app.autocomplete.visible {
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
            match code {
                KeyCode::Esc => {
                    app.autocomplete.visible = false;
                    return Ok(());
                }
                KeyCode::Up => {
                    app.autocomplete.selected_idx = app.autocomplete.selected_idx.saturating_sub(1);
                    return Ok(());
                }
                KeyCode::Down => {
                    if app.autocomplete.selected_idx + 1 < app.autocomplete.options.len() {
                        app.autocomplete.selected_idx += 1;
                    }
                    return Ok(());
                }
                KeyCode::Tab | KeyCode::Enter => {
                    if let Some(opt) = app.autocomplete.options.get(app.autocomplete.selected_idx).cloned() {
                        app.autocomplete.visible = false;
                        let active = app.active_editor;
                        app.editors[active].insert_snippet(&opt.body, app.autocomplete.prefix_len);
                    }
                    return Ok(());
                }
                _ => {
                    app.autocomplete.visible = false;
                }
            }
        }
    }

    if app.prompt != crate::app::PromptType::None {
        return crate::input::prompt_keys::handle(app, event);
    }

    if let Event::Paste(ref text) = event {
        let active = app.active_editor;
        app.editors[active].paste(text);
        return Ok(());
    }

    crate::input::editor_keys::handle(app, event)
}
