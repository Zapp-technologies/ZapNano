use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::editor::{buffer, cursor, highlight};
use crate::plugins::api::PluginRegistry;

pub struct Editor {
    pub buffer: buffer::Buffer,
    pub cursor: cursor::Cursor,
    pub highlighter: highlight::Highlighter,
    pub undo_stack: Vec<(ropey::Rope, cursor::Cursor, bool)>,
    pub redo_stack: Vec<(ropey::Rope, cursor::Cursor, bool)>,
}

impl Editor {
    pub fn new(filepath: Option<PathBuf>, registry: Option<Arc<Mutex<PluginRegistry>>>) -> Result<Self> {
        let buffer = buffer::Buffer::new(filepath.clone())?;
        let cursor = cursor::Cursor::new();
        let highlighter = highlight::Highlighter::new(filepath.as_ref().map(|p| p.as_path()), registry)?;
        let mut editor = Self {
            buffer,
            cursor,
            highlighter,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        };
        editor.load_persistent_history();
        Ok(editor)
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.save_persistent_history();
    }
}
