use crate::app::{App, PromptType};
use crate::input::keybindings::{map_key_event, Command};
use crossterm::event::Event;
use anyhow::Result;

pub fn handle(app: &mut App, event: Event) -> Result<()> {
    let command = map_key_event(event);
    
    let setup_selection = |app: &mut App, shift: bool| {
        let active = app.active_editor;
        let editor = &mut app.editors[active];
        if shift {
            if editor.cursor.selection_start.is_none() {
                editor.cursor.selection_start = Some((editor.cursor.line_idx, editor.cursor.char_idx));
            }
        } else {
            editor.cursor.selection_start = None;
        }
    };
    
    match command {
        Command::InsertChar(c) => {
            let active = app.active_editor;
            app.editors[active].insert_char(c);

            let cursor_line = app.editors[active].cursor.line_idx;
            let cursor_char = app.editors[active].cursor.char_idx;
            if cursor_char >= 3 {
                let text = app.editors[active].buffer.text.line(cursor_line).to_string();
                let chars: Vec<char> = text.chars().collect();
                if chars.len() >= cursor_char {
                    if chars[cursor_char - 3] == '!' && chars[cursor_char - 2] == '.' && chars[cursor_char - 1] == ' ' {
                        app.editors[active].undo();
                        app.editors[active].undo();
                        app.editors[active].undo();
                        app.editors[active].redo_stack.clear();

                        app.prompt = PromptType::Command;
                        app.prompt_input.clear();
                    }
                }
            }
        }
        Command::InsertNewline => {
            let active = app.active_editor;
            app.editors[active].insert_newline();
        }
        Command::InsertTab => {
            let active = app.active_editor;
            app.editors[active].insert_tab(app.settings.tab_size);
        }
        Command::DeleteBackwards => {
            let active = app.active_editor;
            app.editors[active].delete_backwards();
        }
        Command::DeleteForward => {
            let active = app.active_editor;
            app.editors[active].delete_forward();
        }
        Command::MoveUp(shift) => {
            setup_selection(app, shift);
            let active = app.active_editor;
            let editor = &mut app.editors[active];
            editor.cursor.move_up(&editor.buffer);
        }
        Command::MoveDown(shift) => {
            setup_selection(app, shift);
            let active = app.active_editor;
            let editor = &mut app.editors[active];
            editor.cursor.move_down(&editor.buffer);
        }
        Command::MoveLeft(shift) => {
            setup_selection(app, shift);
            let active = app.active_editor;
            let editor = &mut app.editors[active];
            editor.cursor.move_left(&editor.buffer);
        }
        Command::MoveRight(shift) => {
            setup_selection(app, shift);
            let active = app.active_editor;
            let editor = &mut app.editors[active];
            editor.cursor.move_right(&editor.buffer);
        }
        Command::SelectAll => {
            let active = app.active_editor;
            app.editors[active].select_all();
        }
        Command::Undo => {
            let active = app.active_editor;
            app.editors[active].undo();
        }
        Command::Redo => {
            let active = app.active_editor;
            app.editors[active].redo();
        }
        Command::Search => {
            app.prompt = PromptType::Search;
            app.prompt_input.clear();
            app.search_results.clear();
            app.search_idx = 0;
        },
        Command::Save => {
            let active = app.active_editor;
            if app.editors[active].buffer.filepath.is_none() {
                app.prompt = PromptType::SaveAs;
                app.prompt_input.clear();
            } else {
                app.save()?;
            }
        },
        Command::Quit => app.try_quit_all(),
        Command::Escape => {
            let active = app.active_editor;
            let editor = &app.editors[active];
            if editor.buffer.filepath.is_none() && !editor.buffer.modified && editor.buffer.text.len_chars() <= 1 {
                app.show_welcome = true;
            }
        },
        Command::PrevTab => {
            if app.active_editor > 0 {
                app.active_editor -= 1;
            } else {
                app.active_editor = app.editors.len() - 1;
            }
        },
        Command::NextTab => {
            if app.active_editor + 1 < app.editors.len() {
                app.active_editor += 1;
            } else {
                app.active_editor = 0;
            }
        },
        Command::None => {}
    }
    
    {
        let active = app.active_editor;
        let editor = &mut app.editors[active];
        editor.cursor.snap_to_bounds(&editor.buffer);
    }

    if !app.settings.disable_autocomplete {
        if let Some(registry_arc) = app.lua_engine.as_ref().map(|e| std::sync::Arc::clone(&e.registry)) {
            if let Ok(registry) = registry_arc.lock() {
            let active = app.active_editor;
            let editor = &app.editors[active];
            if let Some(snippets) = registry.snippets.get(&editor.highlighter.extension) {
                let line_str = editor.buffer.text.line(editor.cursor.line_idx).to_string();
                let chars: Vec<char> = line_str.chars().collect();
                let mut prefix = String::new();
                let mut i = editor.cursor.char_idx;
                while i > 0 && (chars[i-1].is_alphanumeric() || chars[i-1] == '@' || chars[i-1] == '_') {
                    prefix.insert(0, chars[i-1]);
                    i -= 1;
                }

                if prefix.len() >= 2 {
                    let options: Vec<_> = snippets.iter()
                        .filter(|s| s.prefix.starts_with(&prefix))
                        .cloned()
                        .collect();
                    if !options.is_empty() {
                        app.autocomplete.options = options;
                        app.autocomplete.selected_idx = 0;
                        app.autocomplete.prefix_len = prefix.len();
                        app.autocomplete.visible = true;
                    } else {
                        app.autocomplete.visible = false;
                    }
                } else {
                }
            }
        }
    }
} else {
    app.autocomplete.visible = false;
}

Ok(())
}
