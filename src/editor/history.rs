use super::Editor;

impl Editor {
    pub fn save_history(&mut self) {
        self.undo_stack.push((self.buffer.text.clone(), self.cursor.clone(), self.buffer.modified));
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some((prev_text, prev_cursor, prev_modified)) = self.undo_stack.pop() {
            self.redo_stack.push((self.buffer.text.clone(), self.cursor.clone(), self.buffer.modified));
            self.buffer.text = prev_text;
            self.cursor = prev_cursor;
            self.buffer.modified = prev_modified;
            let text_str = self.buffer.text.to_string();
            self.highlighter.update(&text_str);
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
        }
    }
}
