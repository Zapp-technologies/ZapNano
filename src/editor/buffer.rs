use anyhow::Result;
use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub struct Buffer {
    pub text: Rope,
    pub filepath: Option<PathBuf>,
    pub modified: bool,
}

impl Buffer {
    pub fn new(filepath: Option<PathBuf>) -> Result<Self> {
        let text = if let Some(path) = &filepath {
            if path.exists() {
                let file = File::open(path)?;
                Rope::from_reader(BufReader::new(file))?
            } else {
                Rope::new()
            }
        } else {
            Rope::new()
        };

        Ok(Self {
            text,
            filepath,
            modified: false,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.filepath {
            let file = File::create(path)?;
            self.text.write_to(BufWriter::new(file))?;
            self.modified = false;
        } else {
            return Err(anyhow::anyhow!("No filepath specified for saving"));
        }
        Ok(())
    }

    pub fn insert_char(&mut self, char_idx: usize, ch: char) {
        if char_idx <= self.text.len_chars() {
            self.text.insert_char(char_idx, ch);
            self.modified = true;
        }
    }

    pub fn insert_string(&mut self, char_idx: usize, s: &str) {
        if char_idx <= self.text.len_chars() {
            self.text.insert(char_idx, s);
            self.modified = true;
        }
    }

    pub fn delete_char(&mut self, char_idx: usize) {
        if char_idx < self.text.len_chars() {
            self.text.remove(char_idx..char_idx + 1);
            self.modified = true;
        }
    }

    pub fn len_lines(&self) -> usize {
        self.text.len_lines()
    }

    pub fn len_chars(&self) -> usize {
        self.text.len_chars()
    }

    pub fn line_len(&self, line_idx: usize) -> usize {
        if line_idx < self.len_lines() {
            self.text.line(line_idx).len_chars()
        } else {
            0
        }
    }

    pub fn line_to_char(&self, line_idx: usize) -> usize {
        self.text.line_to_char(line_idx)
    }
}
