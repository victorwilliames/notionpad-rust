use std::io;
use std::path::PathBuf;

use crate::document::Document;
use crate::metadata::{Cover, DocMetadata, MetadataStore};

#[derive(Clone, Debug)]
pub struct TabState {
    pub id: usize,
    pub document: Document,
    pub path: Option<PathBuf>,
    pub title: String,
    pub icon: String,
    pub cover: Cover,
    pub is_dirty: bool,
}

impl TabState {
    pub fn new_empty(id: usize) -> Self {
        Self {
            id,
            document: Document::new(),
            path: None,
            title: "Sem título".to_string(),
            icon: String::new(),
            cover: Cover::None,
            is_dirty: false,
        }
    }

    pub fn display_title(&self) -> String {
        if self.title.trim().is_empty() {
            "Sem título".to_string()
        } else {
            self.title.clone()
        }
    }
}

pub struct Workspace {
    pub tabs: Vec<TabState>,
    pub active_index: usize,
    next_id: usize,
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

impl Workspace {
    pub fn new() -> Self {
        let first_tab = TabState::new_empty(1);
        Self {
            tabs: vec![first_tab],
            active_index: 0,
            next_id: 2,
        }
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn active_tab(&self) -> &TabState {
        &self.tabs[self.active_index]
    }

    pub fn active_tab_mut(&mut self) -> &mut TabState {
        &mut self.tabs[self.active_index]
    }

    pub fn new_tab(&mut self) -> usize {
        let tab = TabState::new_empty(self.next_id);
        self.next_id += 1;
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
        self.active_index
    }

    pub fn switch_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_index = index;
            true
        } else {
            false
        }
    }

    pub fn open_file(&mut self, path: PathBuf) -> io::Result<usize> {
        // Se a aba atual for nova, limpa e vazia, reutilize-a
        let can_reuse = {
            let current = self.active_tab();
            current.path.is_none()
                && !current.is_dirty
                && current.document.len_chars() == 0
                && current.title == "Sem título"
                && current.icon.is_empty()
                && current.cover == Cover::None
        };

        let doc = Document::open(&path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Sem título")
            .to_string();

        let metadata = MetadataStore::get_for_path(&path);
        let (title, icon, cover) = if let Some(meta) = metadata {
            let t = if meta.title.is_empty() {
                file_name
            } else {
                meta.title
            };
            (t, meta.icon, meta.cover)
        } else {
            (file_name, String::new(), Cover::None)
        };

        let target_idx = if can_reuse {
            self.active_index
        } else {
            let new_tab = TabState::new_empty(self.next_id);
            self.next_id += 1;
            self.tabs.push(new_tab);
            self.tabs.len() - 1
        };

        self.tabs[target_idx] = TabState {
            id: self.tabs[target_idx].id,
            document: doc,
            path: Some(path),
            title,
            icon,
            cover,
            is_dirty: false,
        };

        self.active_index = target_idx;
        Ok(target_idx)
    }

    pub fn save_active(&mut self, custom_path: Option<PathBuf>) -> io::Result<PathBuf> {
        let current_path = {
            let active = self.active_tab();
            custom_path.or_else(|| active.path.clone())
        };

        let Some(path) = current_path else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Caminho não especificado para salvar",
            ));
        };

        let active = self.active_tab_mut();
        active.document.save(&path)?;
        active.path = Some(path.clone());
        active.is_dirty = false;

        // Atualiza o título caso seja o padrão
        if active.title.is_empty() || active.title == "Sem título" {
            active.title = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Sem título")
                .to_string();
        }

        // Persiste metadados do documento
        let meta = DocMetadata {
            title: active.title.clone(),
            icon: active.icon.clone(),
            cover: active.cover.clone(),
            cover_offset_y: 0.0,
        };
        MetadataStore::save_for_path(&path, &meta);

        Ok(path)
    }

    pub fn close_tab(&mut self, index: usize) -> usize {
        if index < self.tabs.len() {
            self.tabs.remove(index);
        }

        if self.tabs.is_empty() {
            let new_tab = TabState::new_empty(self.next_id);
            self.next_id += 1;
            self.tabs.push(new_tab);
            self.active_index = 0;
        } else if self.active_index >= self.tabs.len() {
            self.active_index = self.tabs.len() - 1;
        } else if index < self.active_index {
            self.active_index -= 1;
        }

        self.active_index
    }

    pub fn update_active_text(&mut self, text: &str) {
        let active = self.active_tab_mut();
        active.document = Document::from_text(text);
        active.is_dirty = true;
    }

    pub fn update_active_title(&mut self, title: &str) {
        let active = self.active_tab_mut();
        active.title = title.to_string();
        active.is_dirty = true;
    }

    pub fn set_active_icon(&mut self, icon: &str) {
        let active = self.active_tab_mut();
        active.icon = icon.to_string();
        active.is_dirty = true;
    }

    pub fn set_active_cover(&mut self, cover: Cover) {
        let active = self.active_tab_mut();
        active.cover = cover;
        active.is_dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inicializa_com_uma_aba_padrao() {
        let ws = Workspace::new();
        assert_eq!(ws.tab_count(), 1);
        assert_eq!(ws.active_tab().title, "Sem título");
        assert!(!ws.active_tab().is_dirty);
    }

    #[test]
    fn adiciona_e_alterna_entre_abas() {
        let mut ws = Workspace::new();
        let idx1 = ws.new_tab();
        assert_eq!(ws.tab_count(), 2);
        assert_eq!(ws.active_index, idx1);

        ws.update_active_title("Aba 2");
        assert_eq!(ws.active_tab().title, "Aba 2");

        ws.switch_tab(0);
        assert_eq!(ws.active_tab().title, "Sem título");
    }

    #[test]
    fn fechar_aba_garante_que_sempre_resta_ao_menos_uma() {
        let mut ws = Workspace::new();
        ws.close_tab(0);
        assert_eq!(ws.tab_count(), 1);
        assert_eq!(ws.active_tab().title, "Sem título");
    }
}
