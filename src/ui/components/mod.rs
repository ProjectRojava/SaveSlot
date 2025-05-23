//! UI Components for Rojava SaveSlot
//!
//! This module contains all the reusable UI components built with Iced.

pub mod ai_assistant;
pub mod connection_panel;
pub mod dialogs;
pub mod query_editor;
pub mod results_panel;
pub mod schema_visualizer;
pub mod sidebar;
pub mod status_bar;
pub mod tabs;
pub mod toolbar;
pub mod result_view;
pub mod schema_view;

// Re-export commonly used components
pub use ai_assistant::AIAssistant;
pub use connection_panel::ConnectionPanel;
pub use dialogs::*;
pub use query_editor::QueryEditor;
pub use results_panel::ResultsPanel;
pub use schema_visualizer::SchemaVisualizer;
pub use sidebar::Sidebar;
pub use status_bar::StatusBar;
pub use tabs::{TabView, TabId};
pub use toolbar::Toolbar;
pub use result_view::ResultView;
pub use schema_view::SchemaView;

use iced::widget::{Button, Container, Text};
use iced::{Element, Length};

/// Common button styles and utilities
pub mod button_styles {
    use iced::widget::button;
    use iced::{Background, Border, Color, Shadow, Vector};

    pub fn primary() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.6, 1.0))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        }
    }

    pub fn secondary() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            text_color: Color::from_rgb(0.2, 0.2, 0.2),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
        }
    }

    pub fn danger() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.9, 0.2, 0.2))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        }
    }
}

/// Common container styles
pub mod container_styles {
    use iced::widget::container;
    use iced::{Background, Border, Color, Shadow, Vector};

    pub fn card() -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            text_color: None,
        }
    }

    pub fn sidebar() -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            border: Border {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            text_color: None,
        }
    }

    pub fn panel() -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
            border: Border {
                color: Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
            text_color: None,
        }
    }
}

/// Utility functions for creating common UI elements
pub fn create_icon_button<'a, Message>(
    icon: &'a str,
    _tooltip: &'a str,
    message: Message,
) -> Button<'a, Message>
where
    Message: Clone,
{
    Button::new(Text::new(icon).size(16))
        .style(|_theme, _status| button_styles::primary())
        .padding(8)
        .on_press(message)
}

pub fn create_primary_button<'a, Message>(
    label: &'a str,
    message: Option<Message>,
) -> Button<'a, Message>
where
    Message: Clone,
{
    let button = Button::new(Text::new(label))
        .style(|_theme, _status| button_styles::primary())
        .padding([8, 16]);

    if let Some(msg) = message {
        button.on_press(msg)
    } else {
        button
    }
}

pub fn create_card_container<'a, Message>(content: Element<'a, Message>) -> Container<'a, Message> {
    Container::new(content)
        .style(|_theme| container_styles::card())
        .padding(16)
        .width(Length::Fill)
}

/// Common spacing constants
pub mod spacing {
    pub const SMALL: u16 = 4;
    pub const MEDIUM: u16 = 8;
    pub const LARGE: u16 = 16;
    pub const EXTRA_LARGE: u16 = 24;
}

/// Common size constants
pub mod sizes {
    pub const SIDEBAR_WIDTH: u16 = 250;
    pub const TOOLBAR_HEIGHT: u16 = 40;
    pub const STATUS_BAR_HEIGHT: u16 = 24;
    pub const BUTTON_HEIGHT: u16 = 32;
}
