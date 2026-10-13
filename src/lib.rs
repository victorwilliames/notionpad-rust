//! Motor de documentos compartilhável entre desktop e mobile.
//!
//! Nesta primeira etapa, o buffer é baseado em Ropey e independe da interface.

use ropey::Rope;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::ops::Range;
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct Document {
    text: Rope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditError {
    InvalidCharRange,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_text(text: &str) -> Self {
        Self {
            text: Rope::from_str(text),
        }
    }

    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = BufReader::new(File::open(path)?);
        Ok(Self {
            text: Rope::from_reader(file)?,
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let file = File::create(path)?;
        let mut output = BufWriter::new(file);
        for chunk in self.text.chunks() {
            output.write_all(chunk.as_bytes())?;
        }
        output.flush()
    }

    pub fn as_string(&self) -> String {
        self.text.to_string()
    }

    /// Índices são caracteres Unicode, não bytes.
    pub fn insert(&mut self, char_index: usize, text: &str) -> Result<(), EditError> {
        if char_index > self.text.len_chars() {
            return Err(EditError::InvalidCharRange);
        }
        self.text.insert(char_index, text);
        Ok(())
    }

    pub fn remove(&mut self, range: Range<usize>) -> Result<(), EditError> {
        if range.start > range.end || range.end > self.text.len_chars() {
            return Err(EditError::InvalidCharRange);
        }
        self.text.remove(range);
        Ok(())
    }

    pub fn len_chars(&self) -> usize {
        self.text.len_chars()
    }

    pub fn line_count(&self) -> usize {
        self.text.len_lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suporta_unicode_sem_confundir_bytes_e_caracteres() {
        let mut doc = Document::from_text("Olá 🦀");
        assert_eq!(doc.len_chars(), 5);
        doc.insert(4, "Rust ").unwrap();
        assert_eq!(doc.as_string(), "Olá Rust 🦀");
        doc.remove(4..9).unwrap();
        assert_eq!(doc.as_string(), "Olá 🦀");
    }

    #[test]
    fn verifica_indices_invalidos() {
        let mut doc = Document::from_text("abc");
        assert_eq!(doc.insert(4, "x"), Err(EditError::InvalidCharRange));
        assert_eq!(doc.remove(2..5), Err(EditError::InvalidCharRange));
        assert_eq!(doc.as_string(), "abc");
    }

    #[test]
    fn conta_linhas_sem_revarrer_todo_o_documento() {
        let doc = Document::from_text("primeira\nsegunda\nterceira");
        assert_eq!(doc.line_count(), 3);
    }
}
