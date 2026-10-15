use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, scrollable, space, text, text_editor, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme};
use notionpad_core::Workspace;

#[derive(Debug, Clone)]
enum Message {
    EditorAction(text_editor::Action),
    TitleChanged(String),
    NewTab,
    SelectTab(usize),
    CloseTab(usize),
}

struct NotionPadIced {
    workspace: Workspace,
    editor_content: text_editor::Content,
    title_input: String,
    status_msg: String,
}

impl NotionPadIced {
    fn new() -> (Self, Task<Message>) {
        let mut ws = Workspace::new();
        let initial_text = "Bem-vindo ao NotionPad (Versão Iced 0.14 Engine)!\n\nEste motor foi desenvolvido pela comunidade Rust e é a base da interface COSMIC Desktop (System76).\n\nExperimente digitar, criar parágrafos e testar a usabilidade nativa do Rust aqui.";
        ws.active_tab_mut().document.set_text(initial_text);

        (
            Self {
                workspace: ws,
                editor_content: text_editor::Content::with_text(initial_text),
                title_input: String::from("Bem-vindo ao NotionPad"),
                status_msg: String::from("Pronto"),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        format!("NotionPad [Iced Engine] - {}", self.title_input)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorAction(action) => {
                self.editor_content.perform(action);
                let text = self.editor_content.text();
                self.workspace.active_tab_mut().document.set_text(&text);
                Task::none()
            }
            Message::TitleChanged(new_title) => {
                self.title_input = new_title.clone();
                self.workspace.active_tab_mut().title = new_title;
                Task::none()
            }
            Message::NewTab => {
                let id = self.workspace.new_tab();
                self.title_input = self.workspace.active_tab().title.clone();
                self.editor_content = text_editor::Content::new();
                self.status_msg = format!("Nova nota criada (ID #{})", id + 1);
                Task::none()
            }
            Message::SelectTab(index) => {
                self.workspace.switch_tab(index);
                self.title_input = self.workspace.active_tab().title.clone();
                let current_text = self.workspace.active_tab().document.as_string();
                self.editor_content = text_editor::Content::with_text(&current_text);
                Task::none()
            }
            Message::CloseTab(index) => {
                self.workspace.close_tab(index);
                self.title_input = self.workspace.active_tab().title.clone();
                let current_text = self.workspace.active_tab().document.as_string();
                self.editor_content = text_editor::Content::with_text(&current_text);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // 1. Top Bar with Tabs
        let logo = container(
            text("N")
                .size(13)
                .color(Color::WHITE),
        )
        .padding(Padding::from([2, 6]))
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb8(35, 131, 226))),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

        let mut tab_row = row![logo].spacing(6).align_y(Alignment::Center);

        for (idx, tab) in self.workspace.tabs.iter().enumerate() {
            let is_active = idx == self.workspace.active_index;
            let label_str = format!("{} {}{}", tab.icon, tab.title, if tab.is_dirty { " ●" } else { "" });

            let tab_btn = button(
                row![
                    text(label_str).size(12).color(if is_active { Color::WHITE } else { Color::from_rgb8(180, 180, 180) }),
                    space().width(6),
                    button(text("×").size(11))
                        .padding(0)
                        .on_press(Message::CloseTab(idx))
                        .style(|_theme, _status| button::Style {
                            background: None,
                            text_color: Color::from_rgb8(150, 150, 150),
                            ..Default::default()
                        }),
                ]
                .align_y(Alignment::Center),
            )
            .padding(Padding::from([4, 8]))
            .on_press(Message::SelectTab(idx))
            .style(move |_theme, _status| button::Style {
                background: Some(Background::Color(if is_active {
                    Color::from_rgb8(38, 38, 38)
                } else {
                    Color::from_rgb8(28, 28, 28)
                })),
                border: Border {
                    radius: 6.0.into(),
                    width: 1.0,
                    color: if is_active { Color::from_rgb8(55, 55, 55) } else { Color::TRANSPARENT },
                },
                ..Default::default()
            });

            tab_row = tab_row.push(tab_btn);
        }

        let new_tab_btn = button(text("+").size(15))
            .padding(Padding::from([2, 8]))
            .on_press(Message::NewTab)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb8(35, 35, 35))),
                text_color: Color::from_rgb8(180, 180, 180),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

        tab_row = tab_row.push(new_tab_btn);

        let top_bar = container(
            row![
                tab_row,
                space().width(Length::Fill),
                text("COSMIC Iced 0.14 Engine").size(11).color(Color::from_rgb8(120, 120, 120)),
            ]
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([6, 10]))
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb8(24, 24, 24))),
            border: Border {
                width: 1.0,
                color: Color::from_rgb8(35, 35, 35),
                ..Default::default()
            },
            ..Default::default()
        });

        // 2. Cover Banner (Sunset Coral)
        let cover_banner = container(space().width(Length::Fill).height(210))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 75, 43))),
                ..Default::default()
            });

        // 3. Document Canvas (Emoji Badge + Title + Editor)
        let emoji_badge = container(
            text("😍").size(42),
        )
        .width(76)
        .height(76)
        .center_x(76)
        .center_y(76)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb8(25, 25, 25))),
            border: Border {
                radius: 38.0.into(),
                width: 3.0,
                color: Color::from_rgb8(25, 25, 25),
            },
            ..Default::default()
        });

        let title_input_widget = text_input(
            "Sem título",
            &self.title_input,
        )
        .on_input(Message::TitleChanged)
        .size(36)
        .padding(0)
        .style(|_theme, _status| text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
            icon: Color::TRANSPARENT,
            placeholder: Color::from_rgb8(100, 100, 100),
            value: Color::WHITE,
            selection: Color::from_rgba8(35, 131, 226, 0.4),
        });

        let editor_widget = text_editor(&self.editor_content)
            .placeholder("Clique na barra de espaço para ativar a IA ou \"/\" para acessar os comandos")
            .on_action(Message::EditorAction)
            .height(Length::Fill)
            .padding(Padding::new(0.0));

        let doc_content = column![
            emoji_badge,
            space().height(12),
            title_input_widget,
            space().height(16),
            editor_widget,
        ]
        .width(860)
        .spacing(0);

        let centered_doc = container(doc_content)
            .width(Length::Fill)
            .align_x(Horizontal::Center);

        let main_scroller = scrollable(
            column![
                cover_banner,
                centered_doc,
            ]
            .width(Length::Fill)
        )
        .height(Length::Fill);

        // 4. Status Bar
        let pos = self.editor_content.cursor().position;
        let total_chars = self.workspace.active_tab().document.len_chars();

        let status_bar = container(
            row![
                text(&self.status_msg).size(11).color(Color::from_rgb8(150, 150, 150)),
                space().width(Length::Fill),
                text(format!("Ln {}, Col {}", pos.line + 1, pos.column + 1)).size(11).color(Color::from_rgb8(150, 150, 150)),
                space().width(16),
                text(format!("{} caracteres", total_chars)).size(11).color(Color::from_rgb8(150, 150, 150)),
                space().width(16),
                text("UTF-8").size(11).color(Color::from_rgb8(150, 150, 150)),
                space().width(16),
                text("Windows (CRLF)").size(11).color(Color::from_rgb8(150, 150, 150)),
            ]
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([4, 10]))
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb8(20, 20, 20))),
            border: Border {
                width: 1.0,
                color: Color::from_rgb8(32, 32, 32),
                ..Default::default()
            },
            ..Default::default()
        });

        column![
            top_bar,
            main_scroller,
            status_bar,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> iced::Result {
    iced::application(NotionPadIced::new, NotionPadIced::update, NotionPadIced::view)
        .title(NotionPadIced::title)
        .theme(NotionPadIced::theme)
        .centered()
        .run()
}
