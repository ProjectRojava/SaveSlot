//! Results Panel Component - Displays query results

use iced::widget::{Column, Container, Text};
use iced::{Element, Length};

use super::{container_styles, spacing};
use crate::ui::messages::Message;

pub struct ResultsPanel {
    results: Option<Vec<Vec<String>>>,
    is_loading: bool,
}

impl ResultsPanel {
    pub fn new() -> Self {
        Self {
            results: None,
            is_loading: false,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content = if self.is_loading {
            Column::new().push(Text::new("Loading results..."))
        } else if let Some(results) = &self.results {
            Column::new().push(Text::new(format!("{} rows returned", results.len())))
        } else {
            Column::new().push(Text::new("No results to display"))
        };

        Container::new(content)
            .style(|_theme| container_styles::panel())
            .padding(spacing::MEDIUM)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for ResultsPanel {
    fn default() -> Self {
        Self::new()
    }
}
