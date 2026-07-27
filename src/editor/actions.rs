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

    pub fn copy_selection(&self) -> Option<String> {
        if let Some((start, end)) = self.cursor.get_selection_char_bounds(&self.buffer) {
            let mut selected = String::new();
            for i in start..end {
                if i < self.buffer.text.len_chars() {
                    selected.push(self.buffer.text.char(i));
                }
            }
            return Some(selected);
        }
        None
    }

    pub fn insert_char(&mut self, ch: char) {
        self.save_history();
        self.delete_selection();
        
        let idx = self.cursor.get_absolute_char_idx(&self.buffer);

        // If the user types a closing character that already sits right at the
        // cursor (because it was auto-inserted earlier by the pairing logic
        // below), just step over it instead of inserting a duplicate. Without
        // this, typing `"cms.db"` or `{}` by hand ends up producing extra
        // trailing `"` / `}` characters in the buffer, since every closing
        // keystroke would blindly insert a brand new character next to the
        // one that's already there.
        //
        // THIS CHECK MUST STAY. Removing it (as happened once already) brings
        // back the duplicate-quote/duplicate-brace bug.
        let is_closer = matches!(ch, ')' | '}' | ']' | '"');
        if is_closer
            && idx < self.buffer.len_chars()
            && self.buffer.text.char(idx) == ch
        {
            self.cursor.move_right(&self.buffer);
            return;
        }

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
    
    pub fn insert_newline(&mut self, indent_config: &crate::settings::IndentPairConfig, tab_size: usize) {
        self.save_history();
        self.delete_selection();
        
        let line = self.buffer.text.line(self.cursor.line_idx).to_string();
        let line_without_newline = if line.ends_with("\r\n") {
            &line[..line.len() - 2]
        } else if line.ends_with('\n') {
            &line[..line.len() - 1]
        } else {
            &line[..]
        };

        let chars: Vec<char> = line_without_newline.chars().collect();
        let char_idx = std::cmp::min(self.cursor.char_idx, chars.len());
        let text_before: String = chars[..char_idx].iter().collect();
        let text_after: String = chars[char_idx..].iter().collect();

        let decision = crate::indent::determine_indent(
            &text_before,
            &text_after,
            indent_config,
            tab_size,
        );

        let idx = self.cursor.get_absolute_char_idx(&self.buffer);

        match decision.move_close {
            Some(detail) => {
                for _ in 0..detail.close_bracket.chars().count() {
                    self.buffer.delete_char(idx);
                }
                let to_insert = format!(
                    "\n{}\n{}{}",
                    decision.new_line_indent,
                    detail.close_line_indent,
                    detail.close_bracket
                );
                self.buffer.insert_string(idx, &to_insert);
                self.cursor.line_idx += 1;
                self.cursor.char_idx = decision.new_line_indent.chars().count();
                self.cursor.visual_x = self.cursor.char_idx;
            }
            None => {
                let to_insert = format!("\n{}", decision.new_line_indent);
                self.buffer.insert_string(idx, &to_insert);
                self.cursor.line_idx += 1;
                self.cursor.char_idx = decision.new_line_indent.chars().count();
                self.cursor.visual_x = self.cursor.char_idx;
            }
        }
        
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
        self.cursor.char_idx = self.buffer.line_len(self.cursor.line_idx);
        self.cursor.visual_x = self.cursor.char_idx;
        self.cursor.snap_to_bounds(&self.buffer);
    }

    pub fn paste(&mut self, text: &str) {
        self.save_history();
        self.delete_selection();
        
        let idx = self.cursor.get_absolute_char_idx(&self.buffer);
        let normalized = sanitize_paste(text);
        self.buffer.insert_string(idx, &normalized);
        
        let new_abs = idx + normalized.chars().count();
        self.cursor.line_idx = self.buffer.text.char_to_line(new_abs);
        self.cursor.char_idx = new_abs - self.buffer.text.line_to_char(self.cursor.line_idx);
        self.cursor.visual_x = self.cursor.char_idx;
        
        let text_contents = self.buffer.text.to_string();
        self.highlighter.update(&text_contents);
    }
}

/// Normalizes pasted text before inserting it into the buffer.
///
/// Two things can pollute a paste in this app:
///
/// 1. Line-ending differences (CRLF/CR vs LF) - always safe to normalize.
/// 2. Terminal screen-scrape chrome: if the user selects text with the
///    mouse directly in the terminal (an OS-level operation this app has
///    no control over) instead of using the app's own Copy command, the
///    OS clipboard ends up containing whatever was *visually rendered* -
///    including the tab bar, gutter line numbers, and status bar - because
///    the terminal has no concept of "app content" vs "UI chrome", only
///    characters on screen.
///
/// The chrome-stripping below only acts on *structural* patterns (a tab
/// index followed by `|`, a status bar ending in `line:col` after a wide
/// gap, a repeating leading line-number gutter) and is intentionally
/// conservative: normal code that merely contains a `|` or starts with a
/// digit is left completely untouched. It cannot and does not try to
/// "fix" corrupted content that isn't recognizable UI chrome (e.g.
/// duplicated characters already baked into the source) - that class of
/// bug is fixed at the source in `insert_char`, not here.
pub fn sanitize_paste(text: &str) -> String {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");

    let raw_lines: Vec<&str> = normalized.lines().collect();
    if raw_lines.is_empty() {
        return String::new();
    }

    let mut lines = Vec::new();
    for (idx, line) in raw_lines.into_iter().enumerate() {
        let mut processed = if idx == 0 {
            strip_tab_bar_prefix(line)
        } else {
            line.to_string()
        };

        processed = processed.trim_end().to_string();

        if idx == 0 && processed.trim().is_empty() && line.contains('|') {
            continue;
        }

        if is_status_bar_line(&processed) {
            continue;
        }

        lines.push(processed);
    }

    strip_gutter_line_numbers(&mut lines);

    let mut result = lines.join("\n");
    if text.ends_with('\n') || text.ends_with('\r') {
        result.push('\n');
    }
    result
}

/// Detects and strips a leading tab-bar fragment such as
/// ` 1 [No Name] | 2 main.py* |    1 def foo():`.
///
/// Tab bar entries always start with a tab index followed by a space and use
/// `|` as a separator between tabs, regardless of what file extension is
/// open - we deliberately don't hardcode extensions here. Whatever comes
/// after the last `|` must itself look like the start of a gutter-numbered
/// content line (starts with a digit, or is empty) before we treat this as
/// a tab bar; otherwise normal text containing a `|` (e.g. `"1 | 2"`) is
/// left untouched.
fn strip_tab_bar_prefix(line: &str) -> String {
    let trimmed_start = line.trim_start();
    let starts_with_digit = trimmed_start
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false);

    if starts_with_digit && line.contains('|') {
        let has_letters_or_brackets = line.split('|')
            .next()
            .map(|s| s.chars().any(|c| c.is_alphabetic() || c == '[' || c == ']'))
            .unwrap_or(false);

        if has_letters_or_brackets {
            if let Some(pos) = line.rfind('|') {
                let remainder = line[(pos + 1)..].trim_start().to_string();
                let remainder_looks_like_content = remainder.is_empty()
                    || remainder
                        .chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false);

                if remainder_looks_like_content {
                    return remainder;
                }
            }
        }
    }

    line.to_string()
}

/// Detects this app's actual status bar line, e.g.:
/// ` src/app.rs [+]                                    12:5 `
///
/// The fingerprint is: a wide gap of fill spaces, ending in a `line:col`
/// pair. (We don't look for `[Normal]`/`[Insert]`/`[Command]` mode
/// indicators - this editor isn't modal, it has no such modes, so that
/// check would never match the real status bar.)
fn is_status_bar_line(line: &str) -> bool {
    if !line.contains("   ") {
        return false;
    }

    let trimmed = line.trim_end();
    let tail: String = trimmed
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit() || *c == ':')
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    if tail.is_empty() || !tail.contains(':') {
        return false;
    }

    let parts: Vec<&str> = tail.split(':').collect();
    parts.len() == 2 && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Strips a single leading gutter number + separator (e.g. `"  12 "`) from
/// one line, if present. Returns `None` if the line doesn't start with a
/// number after trimming leading whitespace.
fn strip_single_gutter(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let num_len = trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
    if num_len == 0 {
        return None;
    }

    let mut chars = trimmed.chars().skip(num_len);
    if let Some(next_c) = chars.clone().next() {
        if next_c == ' ' || next_c == '\t' {
            chars.next();
        }
    }
    Some(chars.collect())
}

fn strip_gutter_line_numbers(lines: &mut [String]) {
    if lines.len() < 2 {
        if let Some(line) = lines.first_mut() {
            if let Some(stripped) = strip_single_gutter(line) {
                *line = stripped;
            }
        }
        return;
    }

    let mut starts_with_number = true;
    let mut first_num = None;
    let mut expected_num = None;
    let mut checked_lines = 0;
    
    for line in lines.iter() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        checked_lines += 1;
        let num_part: String = trimmed.chars().take_while(|c| c.is_ascii_digit()).collect();
        if num_part.is_empty() {
            starts_with_number = false;
            break;
        }
        if let Ok(num) = num_part.parse::<usize>() {
            if let Some(expected) = expected_num {
                if num != expected + 1 && num != expected {
                    starts_with_number = false;
                    break;
                }
            } else {
                first_num = Some(num);
            }
            expected_num = Some(num);
        } else {
            starts_with_number = false;
            break;
        }
    }

    if starts_with_number && first_num.is_some() && checked_lines >= 2 {
        for line in lines.iter_mut() {
            if line.trim_start().is_empty() {
                continue;
            }
            if let Some(stripped) = strip_single_gutter(line) {
                *line = stripped;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_paste() {
        let mut editor = Editor::new(None, None).unwrap();
        editor.paste("hello\nworld");
        assert_eq!(editor.buffer.text.to_string(), "hello\nworld");
        assert_eq!(editor.cursor.line_idx, 1);
        assert_eq!(editor.cursor.char_idx, 5);

        // test overwriting selection
        editor.cursor.selection_start = Some((0, 0));
        editor.paste("goodbye");
        assert_eq!(editor.buffer.text.to_string(), "goodbye");
        assert_eq!(editor.cursor.line_idx, 0);
        assert_eq!(editor.cursor.char_idx, 7);
    }

    #[test]
    fn test_sanitize_paste_normalizes_line_endings() {
        assert_eq!(sanitize_paste("a\r\nb\rc\n"), "a\nb\nc\n");
    }

    #[test]
    fn test_sanitize_paste_leaves_normal_code_untouched() {
        // Normal code that merely starts with a digit or contains a pipe
        // must never be mangled.
        let code = "1 | 2 => println!(\"two\"),\n3 | 4 => println!(\"four\"),";
        assert_eq!(sanitize_paste(code), code);
    }

    #[test]
    fn test_sanitize_paste_strips_screen_scraped_gutter_clean_case() {
        // This is what a real screen-scrape copy of a freshly-typed (i.e.
        // not corrupted by the old autopair bug) file looks like: tab bar +
        // gutter numbers, no duplicated characters.
        let dirty = "1 [No Name] | 2 db.xcx* |          1 database: db {\n   2     engine = \"sqlite\",\n   3     path   = \"cms.db\"\n   4 };";
        let clean = sanitize_paste(dirty);
        let expected = "database: db {\n    engine = \"sqlite\",\n    path   = \"cms.db\"\n};";
        assert_eq!(clean, expected);
    }

    #[test]
    fn test_strip_tab_bar_any_extension() {
        let dirty = "1 main.py | 2 other.js |                    1 def foo():\n   2     pass";
        let clean = sanitize_paste(dirty);
        assert_eq!(clean, "def foo():\n    pass");
    }

    #[test]
    fn test_status_bar_line_is_stripped() {
        let dirty = "1 [No Name] | 2 app.rs* |                                                        1 fn main() {\n   2     println!(\"hi\");\n   3 }\n src/app.rs                                                              3:2 ";
        let clean = sanitize_paste(dirty);
        assert_eq!(clean, "fn main() {\n    println!(\"hi\");\n}");
    }

    #[test]
    fn test_autopair_quote_no_duplicate_on_manual_close() {
        let mut editor = Editor::new(None, None).unwrap();
        for ch in "\"cms.db\"".chars() {
            editor.insert_char(ch);
        }
        assert_eq!(editor.buffer.text.to_string(), "\"cms.db\"");
    }

    #[test]
    fn test_autopair_brace_no_duplicate_on_manual_close() {
        let mut editor = Editor::new(None, None).unwrap();
        for ch in "{}".chars() {
            editor.insert_char(ch);
        }
        assert_eq!(editor.buffer.text.to_string(), "{}");
    }
}