use crate::editor::Editor;
use crate::input::handler::handle_event;
use crate::ui::explorer;
use crate::plugins::engine::LuaEngine;
use crate::settings::Settings;

use anyhow::Result;
use crossterm::{
    event::{self, Event, EnableBracketedPaste, DisableBracketedPaste, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::SetCursorStyle,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::stdout;
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq)]
pub enum PromptType {
    None,
    Command,
    SaveAs,
    Search,
}

#[derive(PartialEq, Clone, Copy)]
pub enum QuitContext {
    None,
    CloseTab(usize),
    QuitAll,
}

pub struct AutocompleteState {
    pub visible: bool,
    pub options: Vec<crate::plugins::api::SnippetRule>,
    pub selected_idx: usize,
    pub prefix_len: usize,
}

pub struct App {
    pub editors: Vec<Editor>,
    pub active_editor: usize,
    pub should_quit: bool,
    pub scroll_offset: usize,
    pub quit_context: QuitContext,
    pub prompt: PromptType,
    pub prompt_input: String,
    pub show_welcome: bool,
    pub show_settings: bool,
    pub show_commands: bool,
    pub show_extensions: bool,
    pub command_history: Vec<String>,
    pub history_idx: Option<usize>,
    pub settings_focus_idx: usize,
    pub extensions_focus_idx: usize,
    pub welcome_idx: usize,
    pub search_results: Vec<usize>,
    pub search_idx: usize,
    pub explorer: explorer::Explorer,
    pub lua_engine: Option<LuaEngine>,
    pub autocomplete: AutocompleteState,
    pub settings: Settings,
}

impl App {
    pub fn new(filepath: Option<PathBuf>, mut settings: Settings) -> Result<Self> {
        if let Some(ref path) = filepath {
            if let Some(path_str) = path.to_str() {
                let mut is_file = true;
                if let Ok(meta) = std::fs::metadata(path_str) {
                     if meta.is_dir() { is_file = false; }
                }
                if is_file {
                     if let Ok(abs) = std::fs::canonicalize(path) {
                         if let Some(abs_str) = abs.to_str() {
                            settings.add_recent(&abs_str.replace("\\\\?\\", ""));
                         }
                     } else {
                         settings.add_recent(path_str);
                     }
                }
            }
        }
        
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let show_welcome = filepath.is_none();
        
        let lua_engine = LuaEngine::new(&settings.disabled_extensions).ok();
        let registry_opt = lua_engine.as_ref().map(|e| std::sync::Arc::clone(&e.registry));
        
        let mut editor = Editor::new(filepath, registry_opt)?;
        let text_contents = editor.buffer.text.to_string();
        editor.highlighter.update(&text_contents);
        
        Ok(Self {
            editors: vec![editor],
            active_editor: 0,
            should_quit: false,
            scroll_offset: 0,
            quit_context: QuitContext::None,
            prompt: PromptType::None,
            prompt_input: String::new(),
            show_welcome,
            show_settings: false,
            show_commands: false,
            show_extensions: false,
            command_history: Vec::new(),
            history_idx: None,
            settings_focus_idx: 0,
            extensions_focus_idx: 0,
            welcome_idx: 1,
            search_results: Vec::new(),
            search_idx: 0,
            explorer: explorer::Explorer::new(root),
            lua_engine,
            autocomplete: AutocompleteState {
                visible: false,
                options: Vec::new(),
                selected_idx: 0,
                prefix_len: 0,
            },
            settings,
        })
    }

    pub fn update_search(&mut self) {
        self.search_results.clear();
        self.search_idx = 0;
        let query = self.prompt_input.trim();
        if query.is_empty() { return; }
        
        let text = self.editors[self.active_editor].buffer.text.to_string();
        self.search_results = text.match_indices(query).map(|(byte_idx, _)| {
            text[..byte_idx].chars().count()
        }).collect();
        
        self.jump_to_search_idx();
    }
    
    pub fn jump_to_search_idx(&mut self) {
        if self.search_results.is_empty() { return; }
        if self.search_idx >= self.search_results.len() {
            self.search_idx = 0;
        }
        let char_idx = self.search_results[self.search_idx];
        let editor = &mut self.editors[self.active_editor];
        
        let line = editor.buffer.text.char_to_line(char_idx);
        let line_start_char = editor.buffer.text.line_to_char(line);
        
        editor.cursor.line_idx = line;
        editor.cursor.char_idx = char_idx - line_start_char;
        editor.cursor.visual_x = editor.cursor.char_idx;
        
        editor.cursor.selection_start = Some((line, editor.cursor.char_idx));
        for _ in 0..self.prompt_input.trim().chars().count() {
            editor.cursor.move_right(&editor.buffer);
        }
    }

    pub fn save(&mut self) -> Result<()> {
        let active = self.active_editor;
        self.editors[active].buffer.save()?;
        self.editors[active].save_persistent_history();
        Ok(())
    }

    pub fn try_quit_all(&mut self) {
        let any_modified = self.editors.iter().any(|e| e.buffer.modified);
        if any_modified {
            self.quit_context = QuitContext::QuitAll;
        } else {
            self.should_quit = true;
        }
    }

    pub fn try_close_tab(&mut self, idx: usize) {
        if idx < self.editors.len() {
            if self.editors[idx].buffer.modified {
                self.quit_context = QuitContext::CloseTab(idx);
            } else {
                self.force_close_tab(idx);
            }
        }
    }

    pub fn force_close_tab(&mut self, idx: usize) {
        if idx < self.editors.len() {
            self.editors.remove(idx);
            if self.editors.is_empty() {
                self.show_welcome = true;
                if let Ok(empty) = crate::editor::Editor::new(None, self.lua_engine.as_ref().map(|e| std::sync::Arc::clone(&e.registry))) {
                    self.editors.push(empty);
                }
                self.active_editor = 0;
            } else {
                if self.active_editor >= self.editors.len() {
                    self.active_editor = self.editors.len() - 1;
                }
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, SetCursorStyle::BlinkingBar, EnableBracketedPaste)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            let active = self.active_editor;
            let editor = &mut self.editors[active];
            if editor.cursor.line_idx < self.scroll_offset {
                self.scroll_offset = editor.cursor.line_idx;
            } else if editor.cursor.line_idx >= self.scroll_offset + terminal.size()?.height.saturating_sub(1) as usize {
                self.scroll_offset = editor.cursor.line_idx - terminal.size()?.height.saturating_sub(2) as usize;
            }

            terminal.draw(|f| {
                crate::ui::app_view::render(f, self);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind != crossterm::event::KeyEventKind::Press {
                    continue;
                }
                
                if self.quit_context != QuitContext::None {
                    if let crossterm::event::KeyCode::Char('y') | crossterm::event::KeyCode::Char('Y') = key.code {
                        match self.quit_context {
                            QuitContext::QuitAll => self.should_quit = true,
                            QuitContext::CloseTab(idx) => {
                                self.force_close_tab(idx);
                                self.quit_context = QuitContext::None;
                            }
                            _ => {}
                        }
                    } else {
                        self.quit_context = QuitContext::None;
                    }
                } else {
                    // Some terminals/multiplexers (tmux, screen, certain SSH
                    // proxies) don't relay bracketed paste even though we
                    // requested it - the whole clipboard then arrives as a
                    // flood of ordinary key events instead of a single
                    // `Event::Paste`. If we typed those one-by-one, every
                    // character would run through auto-pairing and every
                    // newline would run through smart auto-indent, mangling
                    // the pasted content (extra brackets/quotes, drifting
                    // indentation) and completely skipping `sanitize_paste`.
                    //
                    // A human never has more than one keystroke already
                    // sitting in the input queue; a pasted burst does,
                    // because the terminal writes it all at once. So: if
                    // there's already another event queued right behind this
                    // one, treat the run of plain characters/newlines as a
                    // paste and hand it to `Editor::paste` in one shot.
                    let (paste_text, leftover) = drain_keystroke_burst(key, self.settings.disable_paste_grouping)?;
                    if let Some(text) = paste_text {
                        handle_event(self, Event::Paste(text))?;
                    } else {
                        handle_event(self, Event::Key(key))?;
                    }
                    for ev in leftover {
                        if self.quit_context == QuitContext::None {
                            handle_event(self, ev)?;
                        }
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen, SetCursorStyle::DefaultUserShape, DisableBracketedPaste)?;
        Ok(())
    }
}

/// See the comment at the call site in `App::run`. Drains any key events
/// already queued immediately behind `first` and, if they form a run of
/// plain text characters/newlines, returns them joined as a single string
/// (to be routed through `Editor::paste`). Any event encountered that isn't
/// plain text stops the drain and is returned as `leftover` so it still gets
/// handled normally afterwards - no keystroke is ever silently dropped.
fn drain_keystroke_burst(
    first: crossterm::event::KeyEvent,
    disabled: bool,
) -> Result<(Option<String>, Vec<Event>)> {
    if disabled {
        return Ok((None, Vec::new()));
    }

    let as_char = |k: &crossterm::event::KeyEvent| -> Option<char> {
        match k.code {
            KeyCode::Char(c) if k.modifiers.is_empty() || k.modifiers == KeyModifiers::SHIFT => {
                Some(c)
            }
            KeyCode::Enter => Some('\n'),
            _ => None,
        }
    };

    let mut text = match as_char(&first) {
        Some(c) => c.to_string(),
        None => return Ok((None, Vec::new())),
    };

    let mut leftover = Vec::new();
    let mut extra_chars = 0;

    // Use a zero timeout first. If no other events are queued immediately behind,
    // we bypass the loop. This ensures manual typing is completely lag-free.
    if event::poll(Duration::from_millis(0))? {
        // Collect subsequent events with a small poll timeout (10ms) to bridge
        // slow-arriving segments of a terminal paste stream.
        while event::poll(Duration::from_millis(10))? {
            let ev = event::read()?;
            if let Event::Key(k) = ev {
                if k.kind != crossterm::event::KeyEventKind::Press {
                    continue;
                }
                if let Some(c) = as_char(&k) {
                    text.push(c);
                    extra_chars += 1;
                    continue;
                }
            }
            leftover.push(ev);
            break;
        }
    }

    if extra_chars > 0 {
        Ok((Some(text), leftover))
    } else {
        Ok((None, leftover))
    }
}