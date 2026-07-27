#[derive(Clone)]
pub struct Cursor {
    pub line_idx: usize,
    pub char_idx: usize,
    pub visual_x: usize,
    pub selection_start: Option<(usize, usize)>,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            line_idx: 0,
            char_idx: 0,
            visual_x: 0,
            selection_start: None,
        }
    }

    pub fn get_selection_char_bounds(&self, buffer: &crate::editor::buffer::Buffer) -> Option<(usize, usize)> {
        self.selection_start.map(|(sl, sc)| {
            let start_abs = buffer.line_to_char(sl) + sc;
            let current_abs = self.get_absolute_char_idx(buffer);
            if start_abs < current_abs {
                (start_abs, current_abs)
            } else {
                (current_abs, start_abs)
            }
        })
    }

    pub fn move_up(&mut self, buffer: &crate::editor::buffer::Buffer) {
        if self.line_idx > 0 {
            self.line_idx -= 1;
            self.char_idx = std::cmp::min(self.visual_x, buffer.line_len(self.line_idx).saturating_sub(1));
        }
    }

    pub fn move_down(&mut self, buffer: &crate::editor::buffer::Buffer) {
        if self.line_idx + 1 < buffer.len_lines() {
            self.line_idx += 1;
            self.char_idx = std::cmp::min(self.visual_x, buffer.line_len(self.line_idx).saturating_sub(1));
        }
    }

    pub fn move_left(&mut self, buffer: &crate::editor::buffer::Buffer) {
        if self.char_idx > 0 {
            self.char_idx -= 1;
            self.visual_x = self.char_idx;
        } else if self.line_idx > 0 {
            self.line_idx -= 1;
            self.char_idx = buffer.line_len(self.line_idx).saturating_sub(1);
            self.visual_x = self.char_idx;
        }
    }

    pub fn move_right(&mut self, buffer: &crate::editor::buffer::Buffer) {
        let line_len = buffer.line_len(self.line_idx);
        if self.char_idx + 1 < line_len || (self.line_idx + 1 == buffer.len_lines() && self.char_idx < line_len) {
            self.char_idx += 1;
            self.visual_x = self.char_idx;
        } else if self.line_idx + 1 < buffer.len_lines() {
            self.line_idx += 1;
            self.char_idx = 0;
            self.visual_x = self.char_idx;
        }
    }

    pub fn get_absolute_char_idx(&self, buffer: &crate::editor::buffer::Buffer) -> usize {
        buffer.line_to_char(self.line_idx) + self.char_idx
    }

    pub fn snap_to_bounds(&mut self, buffer: &crate::editor::buffer::Buffer) {
        if self.line_idx >= buffer.len_lines() {
            self.line_idx = buffer.len_lines().saturating_sub(1);
            let len = buffer.line_len(self.line_idx);
            self.char_idx = if len > 0 { len - 1 } else { 0 };
            return;
        }
        
        let mut len = buffer.line_len(self.line_idx);
        if self.line_idx + 1 < buffer.len_lines() && len > 0 {
            len -= 1;
        }
        
        if self.char_idx > len {
            self.char_idx = len;
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
