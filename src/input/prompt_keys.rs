use crate::app::{App, PromptType};
use crossterm::event::{Event, KeyCode, KeyEvent};
use anyhow::Result;

pub fn handle(app: &mut App, event: Event) -> Result<()> {
    if let Event::Key(KeyEvent { code, kind, .. }) = event {
        if kind != crossterm::event::KeyEventKind::Press {
            return Ok(());
        }
        match code {
            KeyCode::Enter => {
                let mut input = app.prompt_input.trim().to_string();
                if app.prompt == PromptType::Command {
                    if input == "!!" {
                        if let Some(last_cmd) = app.command_history.last() {
                            input = last_cmd.clone();
                            app.prompt_input = input.clone();
                        } else {
                            app.prompt = PromptType::None;
                            return Ok(());
                        }
                    }
                    if !input.is_empty() {
                        if app.command_history.is_empty() || app.command_history.last().unwrap() != &input {
                            app.command_history.push(input.clone());
                        }
                        app.history_idx = None;
                    }

                    let mut parts = input.splitn(2, ' ');
                    let cmd_word = parts.next().unwrap_or("");
                    let arg = parts.next().unwrap_or("").trim();

                    match cmd_word {
                        "q" => app.try_quit_all(),
                        "s" => { app.save().unwrap_or_default(); app.prompt = PromptType::None; },
                        "s&q" => { app.save().unwrap_or_default(); app.should_quit = true; },
                        "qnos" => app.should_quit = true,
                        "close" | "c" => {
                            app.try_close_tab(app.active_editor);
                            app.prompt = PromptType::None;
                        },
                        "cmds" | "commands" => { app.show_commands = true; app.prompt = PromptType::None; },
                        "settings" => { app.show_settings = true; app.prompt = PromptType::None; },
                        "ext" | "extensions" | "extentions" => { app.show_extensions = true; app.prompt = PromptType::None; },
                        "help" | "menu" | "welcome" => { app.show_welcome = true; app.prompt = PromptType::None; },
                        "new" => {
                            let registry_opt = app.lua_engine.as_ref().map(|e| std::sync::Arc::clone(&e.registry));
                            if let Ok(empty_editor) = crate::editor::Editor::new(None, registry_opt) {
                                app.editors.push(empty_editor);
                                app.active_editor = app.editors.len() - 1;
                            }
                            app.prompt = PromptType::None;
                        },
                        "mdir" => {
                            if !arg.is_empty() {
                                let _ = std::fs::create_dir_all(arg);
                            }
                            app.prompt = PromptType::None;
                        },
                        "cd" => {
                            if !arg.is_empty() {
                                if let Ok(abs_path) = std::fs::canonicalize(arg) {
                                    if std::env::set_current_dir(&abs_path).is_ok() {
                                        app.explorer.current_dir = abs_path;
                                        app.explorer.refresh();
                                    }
                                } else if std::env::set_current_dir(arg).is_ok() {
                                    if let Ok(cwd) = std::env::current_dir() {
                                        app.explorer.current_dir = cwd;
                                        app.explorer.refresh();
                                    }
                                }
                            }
                            app.prompt = PromptType::None;
                        },
                        "exp" | "ls" => {
                            app.explorer.refresh();
                            app.explorer.visible = true;
                            app.prompt = PromptType::None;
                        },
                        "saveas" => {
                            let active = app.active_editor;
                            if !arg.is_empty() {
                                app.editors[active].buffer.filepath = Some(std::path::PathBuf::from(arg));
                                app.save().unwrap_or_default();
                            }
                            app.prompt = PromptType::None;
                        },
                        "rename" => {
                            let active = app.active_editor;
                            if !arg.is_empty() {
                                let new_path = std::path::PathBuf::from(arg);
                                if let Some(old_path) = &app.editors[active].buffer.filepath {
                                    let _ = std::fs::rename(old_path, &new_path);
                                }
                                app.editors[active].buffer.filepath = Some(new_path);
                                app.save().unwrap_or_default();
                            }
                            app.prompt = PromptType::None;
                        },
                        "" => app.prompt = PromptType::None,
                        _ => { app.prompt = PromptType::None; }
                    }
                } else if app.prompt == PromptType::SaveAs {
                    let active = app.active_editor;
                    if !input.is_empty() {
                        app.editors[active].buffer.filepath = Some(std::path::PathBuf::from(input));
                        app.save().unwrap_or_default();
                    }
                    app.prompt = PromptType::None;
                } else if app.prompt == PromptType::Search {
                    if !app.search_results.is_empty() {
                        app.search_idx = (app.search_idx + 1) % app.search_results.len();
                        app.jump_to_search_idx();
                    }
                }
                if !app.should_quit && app.quit_context == crate::app::QuitContext::None && !app.show_welcome && app.prompt != PromptType::Search {
                    app.prompt = PromptType::None;
                }
            }
            KeyCode::Up => {
                if app.prompt == PromptType::Command && !app.command_history.is_empty() {
                    let new_idx = match app.history_idx {
                        Some(idx) => idx.saturating_sub(1),
                        None => app.command_history.len() - 1,
                    };
                    app.history_idx = Some(new_idx);
                    app.prompt_input = app.command_history[new_idx].clone();
                }
            }
            KeyCode::Down => {
                if app.prompt == PromptType::Command {
                    if let Some(idx) = app.history_idx {
                        if idx + 1 < app.command_history.len() {
                            app.history_idx = Some(idx + 1);
                            app.prompt_input = app.command_history[idx + 1].clone();
                        } else {
                            app.history_idx = None;
                            app.prompt_input.clear();
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if app.prompt_input.is_empty() {
                    app.prompt = PromptType::None;
                } else {
                    app.prompt_input.pop();
                    if app.prompt == PromptType::Search {
                        app.update_search();
                    }
                }
            }
            KeyCode::Esc => {
                app.prompt = PromptType::None;
                app.history_idx = None;
            }
            KeyCode::Char(c) => {
                app.prompt_input.push(c);
                if app.prompt == PromptType::Search {
                    app.update_search();
                }
            }
            _ => {}
        }
    }
    Ok(())
}
