//! Dialog Components - Modal dialogs and popups

use iced::widget::{Button, Column, Container, Text};
use iced::{Element, Length};

use super::{button_styles, container_styles, spacing};
use crate::ui::messages::Message;

pub struct ConnectionDialog {
    pub visible: bool,
}

impl ConnectionDialog {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn view(&self) -> Option<Element<Message>> {
        if !self.visible {
            return None;
        }

        let content = Column::new()
            .push(Text::new("New Database Connection").size(18))
            .push(Text::new("Connection form will be here"))
            .push(
                Button::new(Text::new("Cancel"))
                    .style(|_theme, _status| button_styles::secondary())
                    .padding([8, 16]),
            )
            .spacing(spacing::LARGE)
            .align_x(iced::Alignment::Center);

        Some(
            Container::new(content)
                .style(|_theme| container_styles::card())
                .padding(spacing::LARGE)
                .width(Length::Fixed(400.0))
                .into(),
        )
    }
}

impl Default for ConnectionDialog {
    fn default() -> Self {
        Self::new()
    }
}
