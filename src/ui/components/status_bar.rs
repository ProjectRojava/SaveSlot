//! Status Bar Component - Application status information

use iced::widget::{Row, Text};
use iced::{Element, Length};

use super::spacing;
use crate::ui::messages::Message;

pub struct StatusBar {
    status_text: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            status_text: "Ready".to_string(),
        }
    }

    pub fn update_status(&mut self, status: String) {
        self.status_text = status;
    }

    pub fn view(&self) -> Element<Message> {
        Row::new()
            .push(Text::new(&self.status_text).size(12))
            .push(Text::new("Rojava SaveSlot v0.1.0").size(12))
            .spacing(spacing::LARGE)
            .align_y(iced::Alignment::Center)
            .width(Length::Fill)
            .into()
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}
