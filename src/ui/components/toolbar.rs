//! Toolbar Component - Main application toolbar

use iced::widget::{Button, Row, Text};
use iced::{Element, Length};

use super::{button_styles, spacing};
use crate::ui::messages::Message;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(Text::new("File"))
                    .style(|_theme, _status| button_styles::secondary())
                    .padding([4, 8]),
            )
            .push(
                Button::new(Text::new("Edit"))
                    .style(|_theme, _status| button_styles::secondary())
                    .padding([4, 8]),
            )
            .push(
                Button::new(Text::new("View"))
                    .style(|_theme, _status| button_styles::secondary())
                    .padding([4, 8]),
            )
            .push(
                Button::new(Text::new("Tools"))
                    .style(|_theme, _status| button_styles::secondary())
                    .padding([4, 8]),
            )
            .spacing(spacing::SMALL)
            .align_y(iced::Alignment::Center)
            .width(Length::Fill)
            .into()
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}
