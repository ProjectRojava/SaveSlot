//! AI Assistant Component - Chat interface for AI help

use iced::widget::{Column, Container, Text};
use iced::{Element, Length};

use super::{container_styles, spacing};
use crate::ui::messages::Message;

pub struct AIAssistant;

impl AIAssistant {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("🤖 AI Assistant").size(16))
            .push(Text::new("Ask me anything about your database!"))
            .spacing(spacing::MEDIUM);

        Container::new(content)
            .style(|_theme| container_styles::card())
            .padding(spacing::MEDIUM)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for AIAssistant {
    fn default() -> Self {
        Self::new()
    }
}
