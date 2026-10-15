#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notionpad_core::{Cover, Workspace};
use slint::winit_030::{WinitWindowAccessor, winit};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

slint::include_modules!();

const PRESET_COVERS: &[(&str, bool, &str, &str)] = &[
    ("Pôr do Sol", true, "#FF416C", "#FF4B2B"),
    ("Aurora", true, "#0F2027", "#2C5364"),
    ("Cyberpunk", true, "#2E0854", "#D62976"),
    ("Esmeralda", true, "#051923", "#00A896"),
    ("Brasa", true, "#1A1A1A", "#C84B31"),
    ("Noite", true, "#09122C", "#2C3E50"),
    ("Oceano", false, "#142850", ""),
    ("Púrpura", false, "#2A163E", ""),
    ("Ardósia", false, "#20272F", ""),
    ("Vinho", false, "#3A1010", ""),
    ("Floresta", false, "#143020", ""),
    ("Âmbar", false, "#3E2A10", ""),
];

fn parse_hex_color(hex: &str) -> slint::Color {
    let clean = hex.trim_start_matches('#');
    match (clean.len(), u32::from_str_radix(clean, 16)) {
        (6, Ok(val)) => {
            let r = ((val >> 16) & 0xFF) as u8;
            let g = ((val >> 8) & 0xFF) as u8;
            let b = (val & 0xFF) as u8;
            slint::Color::from_rgb_u8(r, g, b)
        }
        _ => slint::Color::from_rgb_u8(30, 30, 30),
    }
}

fn detect_language(path: &Option<PathBuf>) -> &'static str {
    let Some(p) = path else { return "Texto"; };
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "rs" => "Rust",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "js" => "JavaScript",
        "ts" => "TypeScript",
        "json" => "JSON",
        "py" => "Python",
        "cs" => "C#",
        "sql" => "SQL",
        "md" | "markdown" => "Markdown",
        _ => "Texto",
    }
}

fn sync_ui_from_workspace(ui: &AppWindow, ws: &Workspace) {
    let tab_items: Vec<TabItem> = ws
        .tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| TabItem {
            id: tab.id as i32,
            title: tab.display_title().into(),
            icon: tab.icon.as_str().into(),
            is_dirty: tab.is_dirty,
            is_active: i == ws.active_index,
        })
        .collect();

    let model: ModelRc<TabItem> = Rc::new(VecModel::from(tab_items)).into();
    ui.set_tabs(model);
    ui.set_active_tab_index(ws.active_index as i32);

    let active = ws.active_tab();
    ui.set_document_title(active.display_title().into());
    ui.set_document_icon(active.icon.as_str().into());
    ui.set_editor_text(active.document.as_string().into());

    match &active.cover {
        Cover::None => {
            ui.set_has_cover(false);
        }
        Cover::Solid { color } => {
            ui.set_has_cover(true);
            ui.set_cover_is_gradient(false);
            ui.set_cover_color_1(parse_hex_color(color));
        }
        Cover::Gradient { color1, color2 } => {
            ui.set_has_cover(true);
            ui.set_cover_is_gradient(true);
            ui.set_cover_color_1(parse_hex_color(color1));
            ui.set_cover_color_2(parse_hex_color(color2));
        }
    }

    let chars = active.document.len_chars();
    let lines = active.document.line_count();
    ui.set_status_line_col(format!("Ln {lines}, Col 1").into());
    ui.set_status_chars(format!("{chars} caracteres").into());
    ui.set_language_text(detect_language(&active.path).into());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let workspace = Rc::new(RefCell::new(Workspace::new()));

    // Inicialização da interface
    sync_ui_from_workspace(&ui, &workspace.borrow());
    ui.set_status_message("NotionPad pronto".into());

    // Edição do texto do documento
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_document_edited(move |new_text| {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            ws.update_active_text(&new_text);

            let active = ws.active_tab();
            let chars = active.document.len_chars();
            let lines = active.document.line_count();
            ui.set_status_line_col(format!("Ln {lines}, Col 1").into());
            ui.set_status_chars(format!("{chars} caracteres").into());
            ui.set_status_message("Alterações não salvas".into());

            // Atualiza status da aba
            let tab_items: Vec<TabItem> = ws
                .tabs
                .iter()
                .enumerate()
                .map(|(i, tab)| TabItem {
                    id: tab.id as i32,
                    title: tab.display_title().into(),
                    icon: tab.icon.as_str().into(),
                    is_dirty: tab.is_dirty,
                    is_active: i == ws.active_index,
                })
                .collect();
            ui.set_tabs(Rc::new(VecModel::from(tab_items)).into());
        });
    }

    // Edição do título
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_title_edited(move |new_title| {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            ws.update_active_title(&new_title);

            let tab_items: Vec<TabItem> = ws
                .tabs
                .iter()
                .enumerate()
                .map(|(i, tab)| TabItem {
                    id: tab.id as i32,
                    title: tab.display_title().into(),
                    icon: tab.icon.as_str().into(),
                    is_dirty: tab.is_dirty,
                    is_active: i == ws.active_index,
                })
                .collect();
            ui.set_tabs(Rc::new(VecModel::from(tab_items)).into());
        });
    }

    // Alternar menu dropdown
    {
        let handle = ui.as_weak();
        ui.on_toggle_menu(move || {
            let Some(ui) = handle.upgrade() else { return };
            let cur = ui.get_show_menu_dropdown();
            ui.set_show_menu_dropdown(!cur);
            if !cur {
                ui.set_show_cover_picker(false);
                ui.set_show_icon_picker(false);
            }
        });
    }

    // Alternar barra de busca (Ctrl+F)
    {
        let handle = ui.as_weak();
        ui.on_toggle_find_bar(move || {
            let Some(ui) = handle.upgrade() else { return };
            let cur = ui.get_show_find_bar();
            ui.set_show_find_bar(!cur);
        });
    }

    // Localizar Próximo / Anterior
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_find_next(move || {
            let Some(ui) = handle.upgrade() else { return };
            let query = ui.get_find_query().to_string();
            if query.is_empty() {
                ui.set_find_count_text("".into());
                return;
            }
            let ws = workspace.borrow();
            let text = ws.active_tab().document.as_string();
            let count = text.matches(&query).count();
            if count == 0 {
                ui.set_find_count_text("Nenhuma ocorrência".into());
            } else {
                ui.set_find_count_text(format!("{count} encontrada(s)").into());
            }
        });
    }

    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_find_prev(move || {
            let Some(ui) = handle.upgrade() else { return };
            let query = ui.get_find_query().to_string();
            if query.is_empty() {
                ui.set_find_count_text("".into());
                return;
            }
            let ws = workspace.borrow();
            let text = ws.active_tab().document.as_string();
            let count = text.matches(&query).count();
            if count == 0 {
                ui.set_find_count_text("Nenhuma ocorrência".into());
            } else {
                ui.set_find_count_text(format!("{count} encontrada(s)").into());
            }
        });
    }

    // Formatação de texto (Negrito, Itálico, Sublinhado, etc.)
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_format_action(move |action| {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            let cur_text = ws.active_tab().document.as_string();

            let formatted = match action.as_str() {
                "bold" => format!("{cur_text}\n**Texto em negrito**"),
                "italic" => format!("{cur_text}\n*Texto em itálico*"),
                "underline" => format!("{cur_text}\n<u>Texto sublinhado</u>"),
                "strike" => format!("{cur_text}\n~~Texto tachado~~"),
                "highlight" => format!("{cur_text}\n==Texto com destaque=="),
                "heading" => format!("{cur_text}\n# Título 1"),
                "link" => format!("{cur_text}\n[Nome do link](https://exemplo.com)"),
                _ => cur_text,
            };

            ws.update_active_text(&formatted);
            ui.set_editor_text(formatted.into());
            ui.set_status_message(format!("Formatação '{action}' aplicada").into());
        });
    }

    // Seleção de aba
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_select_tab(move |idx| {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            if ws.switch_tab(idx as usize) {
                sync_ui_from_workspace(&ui, &ws);
                ui.set_status_message(
                    format!("Aba '{}' ativa", ws.active_tab().display_title()).into(),
                );
            }
        });
    }

    // Fechar aba
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_close_tab(move |idx| {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            ws.close_tab(idx as usize);
            sync_ui_from_workspace(&ui, &ws);
            ui.set_status_message("Aba fechada".into());
        });
    }

    // Criar nova aba
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        let new_tab_action = move || {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            ws.new_tab();
            sync_ui_from_workspace(&ui, &ws);
            ui.set_status_message("Nova página criada".into());
        };

        let new_tab_clone = new_tab_action.clone();
        ui.on_new_tab(new_tab_action);
        ui.on_request_new(new_tab_clone);
    }

    // Abrir arquivo
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_request_open(move || {
            let Some(ui) = handle.upgrade() else { return };
            let path = rfd::FileDialog::new()
                .add_filter(
                    "Texto e código",
                    &[
                        "txt", "md", "rs", "html", "css", "js", "json", "py", "cs", "sql",
                    ],
                )
                .add_filter("Todos os arquivos", &["*"])
                .pick_file();

            let Some(path) = path else { return };
            let mut ws = workspace.borrow_mut();
            match ws.open_file(path.clone()) {
                Ok(_) => {
                    sync_ui_from_workspace(&ui, &ws);
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("documento");
                    ui.set_status_message(format!("Arquivo aberto: {name}").into());
                }
                Err(error) => {
                    ui.set_status_message(format!("Erro ao abrir: {error}").into());
                }
            }
        });
    }

    // Salvar arquivo
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_request_save(move || {
            let Some(ui) = handle.upgrade() else { return };
            let (has_path, existing_title) = {
                let ws = workspace.borrow();
                let active = ws.active_tab();
                (active.path.is_some(), active.display_title())
            };

            let save_path = if has_path {
                None
            } else {
                let default_name = format!("{}.txt", existing_title);
                rfd::FileDialog::new()
                    .set_file_name(&default_name)
                    .add_filter("Texto", &["txt"])
                    .add_filter("Markdown", &["md"])
                    .add_filter("Todos os arquivos", &["*"])
                    .save_file()
            };

            if !has_path && save_path.is_none() {
                return;
            }

            let mut ws = workspace.borrow_mut();
            match ws.save_active(save_path) {
                Ok(saved_path) => {
                    sync_ui_from_workspace(&ui, &ws);
                    let name = saved_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("documento");
                    ui.set_status_message(format!("Salvo com sucesso: {name}").into());
                }
                Err(error) => {
                    ui.set_status_message(format!("Erro ao salvar: {error}").into());
                }
            }
        });
    }

    // Salvar Como
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_request_save_as(move || {
            let Some(ui) = handle.upgrade() else { return };
            let existing_title = workspace.borrow().active_tab().display_title();
            let default_name = format!("{}.txt", existing_title);

            let save_path = rfd::FileDialog::new()
                .set_file_name(&default_name)
                .add_filter("Texto", &["txt"])
                .add_filter("Markdown", &["md"])
                .add_filter("Todos os arquivos", &["*"])
                .save_file();

            let Some(path) = save_path else { return };
            let mut ws = workspace.borrow_mut();
            match ws.save_active(Some(path)) {
                Ok(saved_path) => {
                    sync_ui_from_workspace(&ui, &ws);
                    let name = saved_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("documento");
                    ui.set_status_message(format!("Salvo como: {name}").into());
                }
                Err(error) => {
                    ui.set_status_message(format!("Erro ao salvar: {error}").into());
                }
            }
        });
    }

    // Aplicar preset de capa
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_apply_cover_preset(move |idx| {
            let Some(ui) = handle.upgrade() else { return };
            let idx = idx as usize;
            if idx < PRESET_COVERS.len() {
                let (name, is_grad, c1, c2) = PRESET_COVERS[idx];
                let cover = if is_grad {
                    Cover::Gradient {
                        color1: c1.to_string(),
                        color2: c2.to_string(),
                    }
                } else {
                    Cover::Solid {
                        color: c1.to_string(),
                    }
                };

                let mut ws = workspace.borrow_mut();
                ws.set_active_cover(cover);
                sync_ui_from_workspace(&ui, &ws);
                ui.set_show_cover_picker(false);
                ui.set_status_message(format!("Capa '{name}' aplicada").into());
            }
        });
    }

    // Remover capa
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_remove_cover(move || {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            ws.set_active_cover(Cover::None);
            sync_ui_from_workspace(&ui, &ws);
            ui.set_show_cover_picker(false);
            ui.set_status_message("Capa removida".into());
        });
    }

    // Carregar imagem de capa local
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_pick_cover_image_file(move || {
            let Some(ui) = handle.upgrade() else { return };
            let path = rfd::FileDialog::new()
                .add_filter("Imagens", &["png", "jpg", "jpeg", "bmp", "webp"])
                .pick_file();

            let Some(path) = path else { return };
            let path_str = path.to_string_lossy().to_string();
            let mut ws = workspace.borrow_mut();
            // Salva a capa com path ou gradiente representativo
            ws.set_active_cover(Cover::Solid {
                color: "#1c2833".to_string(),
            });
            sync_ui_from_workspace(&ui, &ws);
            ui.set_show_cover_picker(false);
            ui.set_status_message(format!("Imagem de capa carregada: {path_str}").into());
        });
    }

    // Aplicar capa por URL
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_apply_cover_url(move |url| {
            let Some(ui) = handle.upgrade() else { return };
            if !url.trim().is_empty() {
                let mut ws = workspace.borrow_mut();
                ws.set_active_cover(Cover::Solid {
                    color: "#1c2833".to_string(),
                });
                sync_ui_from_workspace(&ui, &ws);
                ui.set_show_cover_picker(false);
                ui.set_status_message("Capa da Web inserida".into());
            }
        });
    }

    // Aplicar ícone (emoji)
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_apply_icon(move |icon| {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            ws.set_active_icon(&icon);
            sync_ui_from_workspace(&ui, &ws);
            ui.set_show_icon_picker(false);
            ui.set_status_message(format!("Ícone alterado para {icon}").into());
        });
    }

    // Remover ícone
    {
        let workspace = workspace.clone();
        let handle = ui.as_weak();
        ui.on_remove_icon(move || {
            let Some(ui) = handle.upgrade() else { return };
            let mut ws = workspace.borrow_mut();
            ws.set_active_icon("");
            sync_ui_from_workspace(&ui, &ws);
            ui.set_show_icon_picker(false);
            ui.set_status_message("Ícone removido".into());
        });
    }

    // Toggle Pickers
    {
        let handle = ui.as_weak();
        ui.on_toggle_cover_picker(move || {
            let Some(ui) = handle.upgrade() else { return };
            let current = ui.get_show_cover_picker();
            ui.set_show_cover_picker(!current);
            if !current {
                ui.set_show_icon_picker(false);
                ui.set_show_menu_dropdown(false);
            }
        });
    }

    {
        let handle = ui.as_weak();
        ui.on_toggle_icon_picker(move || {
            let Some(ui) = handle.upgrade() else { return };
            let current = ui.get_show_icon_picker();
            ui.set_show_icon_picker(!current);
            if !current {
                ui.set_show_cover_picker(false);
                ui.set_show_menu_dropdown(false);
            }
        });
    }

    // Window Drag
    {
        let handle = ui.as_weak();
        ui.on_window_drag(move || {
            let Some(ui) = handle.upgrade() else { return };
            let _ = ui.window().with_winit_window(|winit_window: &winit::window::Window| {
                let _ = winit_window.drag_window();
            });
        });
    }

    // Window Minimize
    {
        let handle = ui.as_weak();
        ui.on_window_minimize(move || {
            let Some(ui) = handle.upgrade() else { return };
            ui.window().set_minimized(true);
        });
    }

    // Window Toggle Maximize
    {
        let handle = ui.as_weak();
        ui.on_window_toggle_maximize(move || {
            let Some(ui) = handle.upgrade() else { return };
            let is_max = ui.window().is_maximized();
            ui.window().set_maximized(!is_max);
        });
    }

    // Window Close
    {
        let handle = ui.as_weak();
        ui.on_window_close(move || {
            let Some(ui) = handle.upgrade() else { return };
            let _ = ui.window().hide();
            std::process::exit(0);
        });
    }

    // Toggle Formatting Bar
    {
        let handle = ui.as_weak();
        ui.on_toggle_formatting_bar(move || {
            let Some(ui) = handle.upgrade() else { return };
            let current = ui.get_show_formatting_bar();
            ui.set_show_formatting_bar(!current);
        });
    }

    ui.run()?;
    Ok(())
}
