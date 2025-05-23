//! Connection panel component for managing database connections

use iced::widget::{Button, Column, Container, Row, Text};
use iced::{Element, Length};

use crate::domain::model::{ConnectionId,  DatabaseConnection};
use crate::ui::messages::Message;

pub struct ConnectionPanel {
    connections: Vec<DatabaseConnection>,
    selected_connection: Option<ConnectionId>,
}

impl ConnectionPanel {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            selected_connection: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = Text::new("Database Connections").size(18);

        let add_button =
            Button::new(Text::new("+ Add Connection")).on_press(Message::NewConnectionRequested);

        let header = Row::new()
            .push(title)
            .push(iced::widget::Space::with_width(Length::Fill))
            .push(add_button)
            .spacing(10)
            .align_y(iced::Alignment::Center);

        let connections_list = if self.connections.is_empty() {
            Column::new()
                .push(Text::new("No connections configured").size(14))
                .push(Text::new("Click 'Add Connection' to get started").size(12))
                .spacing(5)
                .align_x(iced::Alignment::Center)
        } else {
            self.connections
                .iter()
                .fold(Column::new().spacing(5), |column, connection| {
                    column.push(self.connection_item(connection))
                })
        };

        let content = Column::new()
            .push(header)
            .push(iced::widget::Rule::horizontal(1))
            .push(connections_list)
            .spacing(10)
            .padding(15);

        Container::new(content)
            .style(|_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.98, 0.98, 0.98,
                ))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.9, 0.9, 0.9),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
                text_color: None,
            })
            .into()
    }

    // Fix: Add explicit lifetime parameter
    fn connection_item<'a>(&'a self, connection: &'a DatabaseConnection) -> Element<'a, Message> {
        let is_selected = self.selected_connection == Some(connection.id());
        let is_connected = connection.is_active();

        let status_indicator =
            Container::new(Text::new(if is_connected { "●" } else { "○" }).size(12));

        let name_text = Text::new(connection.name()).size(14);
        let host_text = Text::new(format!("{}:{}", connection.host(), connection.port())).size(12);

        let info_column = Column::new().push(name_text).push(host_text).spacing(2);

        let connect_button = if is_connected {
            Button::new(Text::new("Disconnect"))
                .on_press(Message::DisconnectRequested(connection.id()))
        } else {
            Button::new(Text::new("Connect")).on_press(Message::ConnectionRequested(connection.id()))
        };

        let row = Row::new()
            .push(status_indicator)
            .push(info_column)
            .push(iced::widget::Space::with_width(Length::Fill))
            .push(connect_button)
            .spacing(10)
            .align_y(iced::Alignment::Center);

        let container = Container::new(row).padding(10).style(move |_theme| {
            iced::widget::container::Style {
                background: Some(iced::Background::Color(if is_selected {
                    iced::Color::from_rgb(0.9, 0.95, 1.0) // Light blue
                } else {
                    iced::Color::WHITE
                })),
                border: iced::Border {
                    color: if is_selected {
                        iced::Color::from_rgb(0.2, 0.6, 1.0) // Blue
                    } else {
                        iced::Color::from_rgb(0.9, 0.9, 0.9) // Light gray
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
                text_color: None,
            }
        });

        Button::new(container)
            .style(|_theme, _status| iced::widget::button::Style {
                background: None,
                text_color: iced::Color::TRANSPARENT,
                border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
            })
            .on_press(Message::ConnectionSelected(connection.id()))
            .into()
    }

    pub fn update_connections(&mut self, connections: Vec<DatabaseConnection>) {
        self.connections = connections;
    }

    pub fn set_selected(&mut self, connection_id: Option<ConnectionId>) {
        self.selected_connection = connection_id;
    }
}

impl Default for ConnectionPanel {
    fn default() -> Self {
        Self::new()
    }
}
