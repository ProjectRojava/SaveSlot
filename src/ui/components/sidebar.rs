//! Sidebar component for navigation and quick actions

use iced::widget::{button, column, container, text};
use iced::{Element, Length};

use crate::ui::messages::Message;

pub struct Sidebar {
    collapsed: bool,
}

impl Sidebar {
    pub fn new() -> Self {
        Self { collapsed: false }
    }

    pub fn view(&self) -> Element<Message> {
        let width = if self.collapsed {
            Length::Fixed(60.0)
        } else {
            Length::Fixed(200.0)
        };

        let toggle_button = button(text(if self.collapsed { "→" } else { "←" }))
            .on_press(Message::SidebarToggled)
            .style(|_theme, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 0.8, 0.8))),
                text_color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.6, 0.6, 0.6),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
            });

        let mut content = column![toggle_button].spacing(10);

        if !self.collapsed {
            content = content
                .push(self.nav_button("Connections", Message::NavigateToConnections))
                .push(self.nav_button("Query Editor", Message::NavigateToQueryEditor))
                .push(self.nav_button("Results", Message::NavigateToResults))
                .push(self.nav_button("Schema", Message::NavigateToSchema))
                .push(self.nav_button("History", Message::NavigateToHistory));
        }

        container(content)
            .padding(10)
            .width(width)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.95, 0.95, 0.95))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
                text_color: None,
            })
            .into()
    }

    // Fixed lifetime issue by using 'static lifetime and cloning the label
    fn nav_button(&self, label: &str, message: Message) -> Element<'static, Message> {
        button(text(label.to_string()).size(14))
            .width(Length::Fill)
            .on_press(message)
            .style(|_theme, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9))),
                text_color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }

    pub fn toggle(&mut self) {
        self.collapsed = !self.collapsed;
    }

    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}