use floem::peniko::Color;
use floem::reactive::{create_rw_signal, SignalGet};
use floem::views::editor::text::WrapMethod;
use floem::views::text_editor::text_editor;
use floem::views::{container, empty, h_stack, label, scroll, text_input, v_stack, Decorators};
use floem::window::WindowConfig;
use floem::IntoView;

fn app_view() -> impl IntoView {
    let title_signal = create_rw_signal(String::from("Bem-vindo ao NotionPad"));
    let char_count = create_rw_signal(36usize);

    // 1. Top Bar / Tabs
    let top_bar = h_stack((
        // Notion Blue Icon
        container(
            label(|| "N").style(|s| {
                s.color(Color::WHITE)
                    .font_size(12.0)
                    .font_weight(floem::text::Weight::BOLD)
            }),
        )
        .style(|s| {
            s.width(22.0)
                .height(22.0)
                .border_radius(4.0)
                .background(Color::rgb8(35, 131, 226))
                .justify_center()
                .items_center()
                .margin_right(8.0)
        }),
        // Tab 1 (Active)
        container(
            h_stack((
                label(|| "😍 ").style(|s| s.font_size(13.0)),
                label(move || title_signal.get()).style(|s| {
                    s.font_size(12.0)
                        .color(Color::rgb8(240, 240, 240))
                }),
                label(|| " ×").style(|s| {
                    s.font_size(13.0)
                        .color(Color::rgb8(150, 150, 150))
                        .margin_left(8.0)
                }),
            ))
            .style(|s| s.items_center()),
        )
        .style(|s| {
            s.height(28.0)
                .padding_horiz(10.0)
                .border_radius(6.0)
                .background(Color::rgb8(38, 38, 38))
                .border(1.0)
                .border_color(Color::rgb8(60, 60, 60))
                .items_center()
        }),
        // Plus button
        container(
            label(|| "+").style(|s| {
                s.font_size(16.0)
                    .color(Color::rgb8(170, 170, 170))
            }),
        )
        .style(|s| {
            s.width(26.0)
                .height(26.0)
                .margin_left(4.0)
                .justify_center()
                .items_center()
                .border_radius(4.0)
                .hover(|s| s.background(Color::rgb8(45, 45, 45)))
        }),
        // Spacer
        empty().style(|s| s.flex_grow(1.0_f32)),
        // Engine indicator
        container(
            label(|| "Rust Floem Engine").style(|s| {
                s.font_size(11.0)
                    .color(Color::rgb8(120, 120, 120))
            }),
        )
        .style(|s| s.margin_right(12.0)),
    ))
    .style(|s| {
        s.height(38.0)
            .width_full()
            .background(Color::rgb8(31, 31, 31))
            .items_center()
            .padding_horiz(8.0)
            .border_bottom(1.0)
            .border_color(Color::rgb8(40, 40, 40))
    });

    // 2. Cover Banner (Sunset Coral)
    let cover_banner = container(empty().style(|s| {
        s.width_full()
            .height(200.0)
            .background(Color::rgb8(255, 75, 43))
    }))
    .style(|s| s.width_full().height(200.0));

    // 3. Document Content (Emoji + Title + Editor)
    let doc_content = v_stack((
        // Emoji Badge (-38px overlap)
        container(label(|| "😍").style(|s| s.font_size(44.0))).style(|s| {
            s.width(76.0)
                .height(76.0)
                .border_radius(38.0)
                .background(Color::rgb8(25, 25, 25))
                .border(3.0)
                .border_color(Color::rgb8(25, 25, 25))
                .justify_center()
                .items_center()
                .margin_top(-38.0)
        }),
        // Page Title Input
        text_input(title_signal).style(|s| {
            s.font_size(36.0)
                .font_weight(floem::text::Weight::BOLD)
                .color(Color::rgb8(255, 255, 255))
                .background(Color::TRANSPARENT)
                .border(0.0)
                .padding(0.0)
                .margin_top(12.0)
                .width_full()
        }),
        // Multi-line Text Editor
        text_editor("Comece a digitar suas notas aqui...")
            .placeholder("Clique na barra de espaço para ativar a IA ou \"/\" para acessar os comandos")
            .editor_style(|s| {
                s.hide_gutter(true)
                    .cursor_color(Color::WHITE)
                    .selection_color(Color::rgba8(35, 131, 226, 120))
                    .wrap_method(WrapMethod::EditorWidth)
                    .scroll_beyond_last_line(false)
            })
            .style(|s| {
                s.width_full()
                    .min_height(450.0)
                    .color(Color::rgb8(235, 235, 235))
                    .font_size(15.0)
                    .margin_top(16.0)
            }),
    ))
    .style(|s| {
        s.width(860.0)
            .padding_bottom(60.0)
            .items_start()
    });

    // Scrollable page area
    let page_scroll = scroll(
        v_stack((
            cover_banner,
            container(doc_content).style(|s| s.width_full().justify_center()),
        ))
        .style(|s| s.width_full()),
    )
    .style(|s| s.size_full().flex_grow(1.0_f32).background(Color::rgb8(25, 25, 25)));

    // 4. Status Bar
    let status_bar = h_stack((
        label(|| "Pronto").style(|s| s.font_size(11.0).color(Color::rgb8(150, 150, 150))),
        empty().style(|s| s.flex_grow(1.0_f32)),
        label(|| "Ln 1, Col 1").style(|s| s.font_size(11.0).color(Color::rgb8(150, 150, 150)).margin_right(16.0)),
        label(move || format!("{} caracteres", char_count.get())).style(|s| {
            s.font_size(11.0)
                .color(Color::rgb8(150, 150, 150))
                .margin_right(16.0)
        }),
        label(|| "UTF-8").style(|s| s.font_size(11.0).color(Color::rgb8(150, 150, 150)).margin_right(16.0)),
        label(|| "Windows (CRLF)").style(|s| s.font_size(11.0).color(Color::rgb8(150, 150, 150))),
    ))
    .style(|s| {
        s.height(24.0)
            .width_full()
            .background(Color::rgb8(20, 20, 20))
            .border_top(1.0)
            .border_color(Color::rgb8(35, 35, 35))
            .items_center()
            .padding_horiz(10.0)
    });

    // Root Container
    v_stack((
        top_bar,
        page_scroll,
        status_bar,
    ))
    .style(|s| {
        s.size_full()
            .flex_col()
            .background(Color::rgb8(25, 25, 25))
    })
}

fn main() {
    floem::Application::new()
        .window(
            move |_| app_view(),
            Some(
                WindowConfig::default()
                    .title("NotionPad")
                    .size((1100.0, 750.0)),
            ),
        )
        .run();
}
