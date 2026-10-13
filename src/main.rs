//! Primeira interface desktop. O núcleo em src/lib.rs não depende do Slint.
//! O editor Slint atual é provisório; o motor incremental/virtualizado virá depois.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notionpad_core::Document;
use slint::ComponentHandle;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

slint::include_modules!();

#[derive(Default)]
struct Session {
    path: Option<PathBuf>,
    dirty: bool,
}

fn display_name(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Sem título")
        .to_owned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let session = Rc::new(RefCell::new(Session::default()));

    {
        let session = session.clone();
        let handle = ui.as_weak();
        ui.on_document_edited(move || {
            session.borrow_mut().dirty = true;
            if let Some(ui) = handle.upgrade() {
                ui.set_status_message("Alterações não salvas".into());
            }
        });
    }

    {
        let session = session.clone();
        let handle = ui.as_weak();
        ui.on_request_new(move || {
            let Some(ui) = handle.upgrade() else { return };
            if session.borrow().dirty {
                ui.set_status_message("Salve a nota atual antes de criar outra.".into());
                return;
            }

            *session.borrow_mut() = Session::default();
            ui.set_document_title("Sem título".into());
            ui.set_editor_text("".into());
            ui.set_status_message("Nova nota".into());
        });
    }

    {
        let session = session.clone();
        let handle = ui.as_weak();
        ui.on_request_open(move || {
            let Some(ui) = handle.upgrade() else { return };
            if session.borrow().dirty {
                ui.set_status_message("Salve a nota atual antes de abrir outro arquivo.".into());
                return;
            }
            let path = rfd::FileDialog::new()
                .add_filter("Texto e código", &["txt", "md", "rs", "html", "css", "js", "json", "py"])
                .pick_file();

            let Some(path) = path else { return };
            match Document::open(&path) {
                Ok(doc) => {
                    ui.set_editor_text(doc.as_string().into());
                    ui.set_document_title(display_name(&path).into());
                    ui.set_status_message("Arquivo aberto".into());
                    *session.borrow_mut() = Session { path: Some(path), dirty: false };
                }
                Err(error) => {
                    ui.set_status_message(format!("Erro ao abrir: {error}").into());
                }
            }
        });
    }

    {
        let session = session.clone();
        let handle = ui.as_weak();
        ui.on_request_save(move || {
            let Some(ui) = handle.upgrade() else { return };
            let existing_path = session.borrow().path.clone();

            let path = existing_path.or_else(|| {
                rfd::FileDialog::new()
                    .set_file_name("Sem título.txt")
                    .add_filter("Texto", &["txt"])
                    .add_filter("Markdown", &["md"])
                    .save_file()
            });

            let Some(path) = path else { return };
            // Nesta fase, o texto de interface é convertido para Rope apenas
            // na operação de salvar; NÃO reconstruímos a Rope a cada tecla.
            let value = ui.get_editor_text().to_string();
            let document = Document::from_text(&value);

            match document.save(&path) {
                Ok(()) => {
                    session.borrow_mut().path = Some(path.clone());
                    session.borrow_mut().dirty = false;
                    ui.set_document_title(display_name(&path).into());
                    ui.set_status_message("Salvo em UTF-8".into());
                }
                Err(error) => {
                    ui.set_status_message(format!("Erro ao salvar: {error}").into());
                }
            }
        });
    }

    ui.run()?;
    Ok(())
}
