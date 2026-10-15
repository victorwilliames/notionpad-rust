use ropey::Rope;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};

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

    pub fn set_text(&mut self, text: &str) {
        self.text = Rope::from_str(text);
    }

    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = BufReader::new(File::open(path)?);
        Ok(Self {
            text: Rope::from_reader(file)?,
        })
    }

    /// Salva o conteúdo em UTF-8 com salvamento seguro (grava em arquivo temporário e substitui).
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let parent = path.parent().unwrap_or_else(|| Path::new("."));

        // Nome temporário único no mesmo diretório para garantir operação atômica de rename
        let temp_path: PathBuf = parent.join(format!(
            ".{}.tmp_{}",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("doc"),
            std::process::id()
        ));

        {
            let file = File::create(&temp_path)?;
            let mut writer = BufWriter::new(file);
            for chunk in self.text.chunks() {
                writer.write_all(chunk.as_bytes())?;
            }
            writer.flush()?;
        }

        // No Windows, fs::rename sobre um arquivo existente pode falhar se não usarmos replace.
        // Tentamos rename direto; se falhar, removemos o destino e renomeamos.
        if fs::rename(&temp_path, path).is_err() {
            let _ = fs::remove_file(path);
            fs::rename(&temp_path, path)?;
        }

        Ok(())
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

    /// Contagem aproximada de palavras sem alocar o texto inteiro na memória
    pub fn word_count(&self) -> usize {
        let mut count = 0;
        let mut in_word = false;
        for ch in self.text.chars() {
            if ch.is_whitespace() {
                in_word = false;
            } else if !in_word {
                in_word = true;
                count += 1;
            }
        }
        count
    }

    /// Converte índice de caractere (0-indexed) em (linha, coluna), ambos 1-indexed
    pub fn char_to_line_col(&self, char_idx: usize) -> (usize, usize) {
        let total_chars = self.text.len_chars();
        let safe_idx = char_idx.min(total_chars);
        let line = self.text.char_to_line(safe_idx);
        let line_start_char = self.text.line_to_char(line);
        let col = safe_idx.saturating_sub(line_start_char);
        (line + 1, col + 1)
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
    fn conta_linhas_e_palavras() {
        let doc = Document::from_text("primeira linha\nsegunda linha com mais palavras\nterceira");
        assert_eq!(doc.line_count(), 3);
        assert_eq!(doc.word_count(), 8);
    }

    #[test]
    fn calcula_linha_e_coluna() {
        let doc = Document::from_text("abc\ndefgh");
        assert_eq!(doc.char_to_line_col(0), (1, 1));
        assert_eq!(doc.char_to_line_col(2), (1, 3));
        assert_eq!(doc.char_to_line_col(4), (2, 1));
        assert_eq!(doc.char_to_line_col(6), (2, 3));
    }
}
