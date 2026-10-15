use floem::peniko::Color;
use floem::reactive::{create_rw_signal, SignalGet};
use floem::style::AlignItems;
use floem::views::{container, drag_window_area, empty, h_stack, label, text_input, v_stack, Decorators};
use floem::window::WindowConfig;
use floem::IntoView;

fn app_view() -> impl IntoView {
    let title_signal = create_rw_signal(String::from("Titulo"));
    let text_signal = create_rw_signal(String::from(""));
    let active_tab = create_rw_signal(0);

    // Top Bar (Tabs + Drag area + Caption controls)
    let top_bar = h_stack((
        // Notion Blue square logo
        container(
            empty()
                .style(|s| {
                    s.width(15.0)
                        .height(15.0)
                        .border_radius(3.0)
                        .background(Color::rgb8(35, 131, 226))
                }),
        )
        .style(|s| {
            s.width(34.0)
                .height(34.0)
                .align_items(AlignItems::Center)
                .justify_center()
        }),

        // Tabs
        h_stack((
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
                            .margin_left(6.0)
                    }),
                ))
                .style(|s| s.align_items(AlignItems::Center)),
            )
            .style(move |s| {
                s.height(28.0)
                    .padding_horiz(10.0)
                    .border_radius(6.0)
                    .background(Color::rgb8(38, 38, 38))
                    .border(1.0)
                    .border_color(Color::rgb8(56, 56, 56))
                    .align_items(AlignItems::Center)
            }),
            // Plus button
            container(
                label(|| "+").style(|s| {
                    s.font_size(16.0)
                        .color(Color::rgb8(150, 150, 150))
                }),
            )
            .style(|s| {
                s.width(28.0)
                    .height(28.0)
                    .justify_center()
                    .align_items(AlignItems::Center)
                    .border_radius(4.0)
            }),
        ))
        .style(|s| s.align_items(AlignItems::Center).gap(4.0)),

        // Draggable window space
        drag_window_area(
            empty().style(|s| s.flex_grow(1.0_f32).height(40.0)),
        ),

        // Caption buttons (—, ⬜, ✕)
        h_stack((
            container(
                label(|| "—").style(|s| s.font_size(11.0).color(Color::rgb8(180, 180, 180))),
            )
            .style(|s| {
                s.width(46.0)
                    .height(40.0)
                    .justify_center()
                    .align_items(AlignItems::Center)
            }),
            container(
                label(|| "⬜").style(|s| s.font_size(10.0).color(Color::rgb8(180, 180, 180))),
            )
            .style(|s| {
                s.width(46.0)
                    .height(40.0)
                    .justify_center()
                    .align_items(AlignItems::Center)
            }),
            container(
                label(|| "✕").style(|s| s.font_size(12.0).color(Color::rgb8(180, 180, 180))),
            )
            .on_click_stop(|_| {
                std::process::exit(0);
            })
            .style(|s| {
                s.width(46.0)
                    .height(40.0)
                    .justify_center()
                    .align_items(AlignItems::Center)
                    .hover(|s| s.background(Color::rgb8(232, 17, 35)))
            }),
        )),
    ))
    .style(|s| {
        s.height(40.0)
            .width_full()
            .background(Color::rgb8(31, 31, 31))
            .align_items(AlignItems::Center)
            .padding_left(8.0)
    });

    // Cover Banner (220px Sunset Coral Gradient)
    let cover_banner = container(
        empty().style(|s| {
            s.width_full()
                .height(220.0)
                .background(Color::rgb8(255, 75, 43))
        }),
    )
    .style(|s| s.width_full().height(220.0));

    // Document Body (860px Centered)
    let doc_container = v_stack((
        // Emoji Badge (-42px overlap)
        container(
            label(|| "😍").style(|s| s.font_size(44.0)),
        )
        .style(|s| {
            s.width(76.0)
                .height(76.0)
                .border_radius(38.0)
                .background(Color::rgb8(25, 25, 25))
                .border(3.0)
                .border_color(Color::rgb8(25, 25, 25))
                .justify_center()
                .align_items(AlignItems::Center)
                .margin_top(-38.0)
        }),

        // Title Box (36px Bold)
        text_input(title_signal)
            .style(|s| {
                s.font_size(36.0)
                    .font_weight(floem::text::Weight::BOLD)
                    .color(Color::rgb8(255, 255, 255))
                    .background(Color::TRANSPARENT)
                    .border(0.0)
                    .padding(0.0)
                    .margin_top(12.0)
            }),

        // Editor Canvas
        text_input(text_signal)
            .placeholder("Clique na barra de espaço para ativar a IA ou \"/\" para acessar os comandos")
            .style(|s| {
                s.font_size(15.0)
                    .color(Color::rgb8(240, 240, 240))
                    .background(Color::TRANSPARENT)
                    .border(0.0)
                    .padding(0.0)
                    .margin_top(16.0)
                    .min_height(400.0)
            }),
    ))
    .style(|s| {
        s.width(860.0)
            .align_items(floem::style::AlignItems::FlexStart)
    });

    // Center the doc container
    let main_scroll = v_stack((
        cover_banner,
        container(doc_container).style(|s| s.width_full().justify_center()),
    ))
    .style(|s| {
        s.flex_grow(1.0_f32)
            .width_full()
            .background(Color::rgb8(25, 25, 25))
    });

    // Root layout
    v_stack((
        top_bar,
        main_scroll,
    ))
    .style(|s| {
        s.width_full()
            .height_full()
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
                    .size((1100.0, 750.0))
                    .show_titlebar(false)
                    .undecorated(true)
                    .undecorated_shadow(true),
            ),
        )
        .run();
}
