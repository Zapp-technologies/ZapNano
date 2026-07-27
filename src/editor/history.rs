use super::Editor;
use crate::editor::cursor::Cursor;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct PersistentHistoryItem {
    pub text: String,
    pub cursor_line: usize,
    pub cursor_char: usize,
    pub cursor_visual_x: usize,
    pub modified: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FileHistory {
    pub undo: Vec<PersistentHistoryItem>,
    pub redo: Vec<PersistentHistoryItem>,
}

fn get_history_path(filepath: &Path) -> PathBuf {
    let mut s = DefaultHasher::new();
    filepath.hash(&mut s);
    let hash = s.finish();
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("zapnano").join("undo").join(format!("{:x}.toml", hash))
}

impl Editor {
    pub fn save_history(&mut self) {
        self.undo_stack.push((self.buffer.text.clone(), self.cursor.clone(), self.buffer.modified));
        self.redo_stack.clear();
        self.save_persistent_history();
    }

    pub fn undo(&mut self) {
        if let Some((prev_text, prev_cursor, prev_modified)) = self.undo_stack.pop() {
            self.redo_stack.push((self.buffer.text.clone(), self.cursor.clone(), self.buffer.modified));
            self.buffer.text = prev_text;
            self.cursor = prev_cursor;
            self.buffer.modified = prev_modified;
            let text_str = self.buffer.text.to_string();
            self.highlighter.update(&text_str);
            self.save_persistent_history();
        }
    }

    pub fn redo(&mut self) {
        if let Some((next_text, next_cursor, next_modified)) = self.redo_stack.pop() {
            self.undo_stack.push((self.buffer.text.clone(), self.cursor.clone(), self.buffer.modified));
            self.buffer.text = next_text;
            self.cursor = next_cursor;
            self.buffer.modified = next_modified;
            let text_str = self.buffer.text.to_string();
            self.highlighter.update(&text_str);
            self.save_persistent_history();
        }
    }

    pub fn load_persistent_history(&mut self) {
        if let Some(ref path) = self.buffer.filepath {
            let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
            let history_path = get_history_path(&canon);
            if history_path.exists() {
                if let Ok(contents) = std::fs::read_to_string(&history_path) {
                    if let Ok(file_hist) = toml::from_str::<FileHistory>(&contents) {
                        self.undo_stack = file_hist
                            .undo
                            .into_iter()
                            .map(|item| {
                                let rope = ropey::Rope::from_str(&item.text);
                                let mut cursor = Cursor::new();
                                cursor.line_idx = item.cursor_line;
                                cursor.char_idx = item.cursor_char;
                                cursor.visual_x = item.cursor_visual_x;
                                (rope, cursor, item.modified)
                            })
                            .collect();

                        self.redo_stack = file_hist
                            .redo
                            .into_iter()
                            .map(|item| {
                                let rope = ropey::Rope::from_str(&item.text);
                                let mut cursor = Cursor::new();
                                cursor.line_idx = item.cursor_line;
                                cursor.char_idx = item.cursor_char;
                                cursor.visual_x = item.cursor_visual_x;
                                (rope, cursor, item.modified)
                            })
                            .collect();
                    }
                }
            }
        }
    }

    pub fn save_persistent_history(&self) {
        if let Some(ref path) = self.buffer.filepath {
            let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
            let history_path = get_history_path(&canon);
            
            if let Some(parent) = history_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            let file_hist = FileHistory {
                undo: self
                    .undo_stack
                    .iter()
                    .map(|(rope, cursor, modified)| PersistentHistoryItem {
                        text: rope.to_string(),
                        cursor_line: cursor.line_idx,
                        cursor_char: cursor.char_idx,
                        cursor_visual_x: cursor.visual_x,
                        modified: *modified,
                    })
                    .collect(),
                redo: self
                    .redo_stack
                    .iter()
                    .map(|(rope, cursor, modified)| PersistentHistoryItem {
                        text: rope.to_string(),
                        cursor_line: cursor.line_idx,
                        cursor_char: cursor.char_idx,
                        cursor_visual_x: cursor.visual_x,
                        modified: *modified,
                    })
                    .collect(),
            };

            if let Ok(toml_str) = toml::to_string(&file_hist) {
                let _ = std::fs::write(&history_path, toml_str);
            }
        }
    }
}
