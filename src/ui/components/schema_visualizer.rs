//! Schema Visualizer Component - Visual database schema representation

use iced::widget::{Column, Container, Text};
use iced::{Element, Length};

use super::{container_styles, spacing};
use crate::ui::messages::Message;

pub struct SchemaVisualizer;

impl SchemaVisualizer {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Schema Visualizer").size(18))
            .push(Text::new(
                "Interactive database schema will be displayed here",
            ))
            .spacing(spacing::MEDIUM);

        Container::new(content)
            .style(|_theme| container_styles::panel())
            .padding(spacing::LARGE)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for SchemaVisualizer {
    fn default() -> Self {
        Self::new()
    }
}
