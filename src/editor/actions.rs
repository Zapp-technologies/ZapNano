use super::Editor;

impl Editor {
    pub fn delete_selection(&mut self) -> bool {
        if let Some((start, end)) = self.cursor.get_selection_char_bounds(&self.buffer) {
            for _ in start..end {
                self.buffer.delete_char(start);
            }
            
            let (sl, sc) = if self.cursor.selection_start.unwrap() < (self.cursor.line_idx, self.cursor.char_idx) {
                self.cursor.selection_start.unwrap()
            } else {
                (self.cursor.line_idx, self.cursor.char_idx)
            };
            
            self.cursor.line_idx = sl;
            self.cursor.char_idx = sc;
            self.cursor.visual_x = sc;
            self.cursor.selection_start = None;
            
            let text_contents = self.buffer.text.to_string();
            self.highlighter.update(&text_contents);
            return true;
        }
        false
    }

    pub fn insert_char(&mut self, ch: char) {
        self.save_history();
        self.delete_selection();
        
        let idx = self.cursor.get_absolute_char_idx(&self.buffer);
        self.buffer.insert_char(idx, ch);
        
        match ch {
            '(' => self.buffer.insert_char(idx + 1, ')'),
            '{' => self.buffer.insert_char(idx + 1, '}'),
            '[' => self.buffer.insert_char(idx + 1, ']'),
            '"' => self.buffer.insert_char(idx + 1, '"'),
            _ => {}
        }
        
        self.cursor.move_right(&self.buffer);
        let text_contents = self.buffer.text.to_string();
        self.highlighter.update(&text_contents);
    }
    
    pub fn insert_newline(&mut self) {
        self.save_history();
        self.delete_selection();
        
        let idx = self.cursor.get_absolute_char_idx(&self.buffer);
        self.buffer.insert_char(idx, '\n');
        self.cursor.line_idx += 1;
        self.cursor.char_idx = 0;
        self.cursor.visual_x = 0;
        let text_contents = self.buffer.text.to_string();
        self.highlighter.update(&text_contents);
    }

    pub fn insert_snippet(&mut self, body: &str, prefix_len: usize) {
        self.save_history();
        for _ in 0..prefix_len {
            self.cursor.move_left(&self.buffer);
            let idx = self.cursor.get_absolute_char_idx(&self.buffer);
            self.buffer.delete_char(idx);
        }
        
        let mut cursor_offset = None;
        for (i, ch) in body.chars().enumerate() {
            if ch == '|' && cursor_offset.is_none() {
                cursor_offset = Some(i);
            } else {
                let idx = self.cursor.get_absolute_char_idx(&self.buffer);
                self.buffer.insert_char(idx, ch);
                if ch == '\n' {
                    self.cursor.line_idx += 1;
                    self.cursor.char_idx = 0;
                    self.cursor.visual_x = 0;
                } else {
                    self.cursor.char_idx += 1;
                    self.cursor.visual_x = self.cursor.char_idx;
                }
            }
        }
        
        if let Some(offset) = cursor_offset {
            let move_back = body.replace("|", "").chars().count() - offset;
            for _ in 0..move_back {
                self.cursor.move_left(&self.buffer);
            }
        }
        
        let text_contents = self.buffer.text.to_string();
        self.highlighter.update(&text_contents);
    }

    pub fn insert_tab(&mut self, tab_size: usize) {
        self.save_history();
        self.delete_selection();
        
        let idx = self.cursor.get_absolute_char_idx(&self.buffer);
        let spaces = " ".repeat(tab_size);
        self.buffer.insert_string(idx, &spaces);
        
        self.cursor.char_idx += tab_size;
        self.cursor.visual_x = self.cursor.char_idx;
        let text_contents = self.buffer.text.to_string();
        self.highlighter.update(&text_contents);
    }

    pub fn delete_backwards(&mut self) {
        self.save_history();
        if self.delete_selection() {
            return;
        }
        
        if self.cursor.line_idx == 0 && self.cursor.char_idx == 0 {
            // Revert history push if nothing changed
            self.undo_stack.pop();
            return;
        }
        self.cursor.move_left(&self.buffer);
        let idx = self.cursor.get_absolute_char_idx(&self.buffer);
        self.buffer.delete_char(idx);
        let text_contents = self.buffer.text.to_string();
        self.highlighter.update(&text_contents);
    }
    
    pub fn delete_forward(&mut self) {
        self.save_history();
        if self.delete_selection() {
            return;
        }
        
        let idx = self.cursor.get_absolute_char_idx(&self.buffer);
        if idx < self.buffer.len_chars() {
            self.buffer.delete_char(idx);
            let text_contents = self.buffer.text.to_string();
            self.highlighter.update(&text_contents);
        } else {
            self.undo_stack.pop();
        }
    }
    
    pub fn select_all(&mut self) {
        self.cursor.selection_start = Some((0, 0));
        self.cursor.line_idx = self.buffer.len_lines().saturating_sub(1);
        self.cursor.char_idx = self.buffer.line_len(self.cursor.line_idx).saturating_sub(1);
        self.cursor.visual_x = self.cursor.char_idx;
        self.cursor.snap_to_bounds(&self.buffer);
    }
}
