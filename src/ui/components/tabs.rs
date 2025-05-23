//! Tab view component for organizing different panels

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};

use crate::ui::messages::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabId {
    Connections,
    QueryEditor,
    Results,
    Schema,
    History,
}

impl TabId {
    pub fn all() -> &'static [TabId] {
        &[
            TabId::Connections,
            TabId::QueryEditor,
            TabId::Results,
            TabId::Schema,
            TabId::History,
        ]
    }

    pub fn title(self) -> &'static str {
        match self {
            TabId::Connections => "Connections",
            TabId::QueryEditor => "Query Editor",
            TabId::Results => "Results",
            TabId::Schema => "Schema",
            TabId::History => "History",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            TabId::Connections => "🔗",
            TabId::QueryEditor => "📝",
            TabId::Results => "📊",
            TabId::Schema => "🗂️",
            TabId::History => "📜",
        }
    }
}

pub struct TabView {
    active_tab: TabId,
    tabs: Vec<TabId>,
}

impl TabView {
    pub fn new() -> Self {
        Self {
            active_tab: TabId::Connections,
            tabs: TabId::all().to_vec(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let tab_bar = self.create_tab_bar();
        let content = self.create_content_area();

        column![tab_bar, content]
            .spacing(0)
            .into()
    }

    fn create_tab_bar(&self) -> Element<Message> {
        let tabs = self.tabs
            .iter()
            .fold(row![].spacing(0), |row_widget, &tab_id| {
                row_widget.push(self.create_tab_button(tab_id))
            });

        container(tabs)
            .style(|_theme| self.tab_bar_style(_theme))
            .width(Length::Fill)
            .into()
    }

    fn create_tab_button(&self, tab_id: TabId) -> Element<Message> {
        let is_active = self.active_tab == tab_id;
        
        let content = row![
            text(tab_id.icon()).size(16),
            text(tab_id.title()).size(14)
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        button(
            container(content)
                .padding([8, 16])
                .style(move |_theme| self.tab_content_style(_theme, is_active))
        )
        .style(move |_theme, status| self.tab_button_style(_theme, status, is_active))
        .on_press(Message::TabChanged(tab_id as usize))
        .into()
    }

    fn create_content_area(&self) -> Element<Message> {
        let content = match self.active_tab {
            TabId::Connections => self.connections_content(),
            TabId::QueryEditor => self.query_editor_content(),
            TabId::Results => self.results_content(),
            TabId::Schema => self.schema_content(),
            TabId::History => self.history_content(),
        };

        container(content)
            .style(|_theme| self.content_area_style(_theme))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn connections_content(&self) -> Element<Message> {
        column![
            text("Database Connections").size(20),
            text("Manage your database connections here").size(14)
        ]
        .spacing(10)
        .into()
    }

    fn query_editor_content(&self) -> Element<Message> {
        column![
            text("SQL Query Editor").size(20),
            text("Write and execute SQL queries here").size(14)
        ]
        .spacing(10)
        .into()
    }

    fn results_content(&self) -> Element<Message> {
        column![
            text("Query Results").size(20),
            text("View query results and data here").size(14)
        ]
        .spacing(10)
        .into()
    }

    fn schema_content(&self) -> Element<Message> {
        column![
            text("Database Schema").size(20),
            text("Explore database structure and relationships").size(14)
        ]
        .spacing(10)
        .into()
    }

    fn history_content(&self) -> Element<Message> {
        column![
            text("Query History").size(20),
            text("Browse your previous queries").size(14)
        ]
        .spacing(10)
        .into()
    }

    // Style methods with correct closure signatures
    fn tab_bar_style(&self, _theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.95, 0.95, 0.95))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            text_color: None,
        }
    }

    fn tab_button_style(
        &self, 
        _theme: &iced::Theme, 
        status: iced::widget::button::Status, 
        is_active: bool
    ) -> iced::widget::button::Style {
        let base_color = if is_active {
            iced::Color::WHITE
        } else {
            iced::Color::from_rgb(0.95, 0.95, 0.95)
        };

        let hover_color = if is_active {
            iced::Color::WHITE
        } else {
            iced::Color::from_rgb(0.9, 0.9, 0.9)
        };

        match status {
            iced::widget::button::Status::Active => iced::widget::button::Style {
                background: Some(iced::Background::Color(base_color)),
                text_color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                border: iced::Border {
                    color: if is_active {
                        iced::Color::from_rgb(0.2, 0.6, 1.0)
                    } else {
                        iced::Color::TRANSPARENT
                    },
                    width: if is_active { 2.0 } else { 0.0 },
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            },
            iced::widget::button::Status::Hovered => iced::widget::button::Style {
                background: Some(iced::Background::Color(hover_color)),
                text_color: iced::Color::from_rgb(0.1, 0.1, 0.1),
                border: iced::Border {
                    color: if is_active {
                        iced::Color::from_rgb(0.2, 0.6, 1.0)
                    } else {
                        iced::Color::from_rgb(0.7, 0.7, 0.7)
                    },
                    width: if is_active { 2.0 } else { 1.0 },
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            },
            iced::widget::button::Status::Pressed => iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.85, 0.85, 0.85))),
                text_color: iced::Color::from_rgb(0.1, 0.1, 0.1),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.2, 0.6, 1.0),
                    width: 2.0,
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            },
            iced::widget::button::Status::Disabled => iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9))),
                text_color: iced::Color::from_rgb(0.6, 0.6, 0.6),
                border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            },
        }
    }

    fn tab_content_style(&self, _theme: &iced::Theme, is_active: bool) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: None,
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow::default(),
            text_color: Some(if is_active {
                iced::Color::from_rgb(0.1, 0.1, 0.1)
            } else {
                iced::Color::from_rgb(0.4, 0.4, 0.4)
            }),
        }
    }

    fn content_area_style(&self, _theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::Border {
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow::default(),
            text_color: None,
        }
    }

    // Public methods
    pub fn set_active_tab(&mut self, tab_id: TabId) {
        self.active_tab = tab_id;
    }

    pub fn active_tab(&self) -> TabId {
        self.active_tab
    }

    pub fn next_tab(&mut self) {
        if let Some(current_index) = self.tabs.iter().position(|&tab| tab == self.active_tab) {
            let next_index = (current_index + 1) % self.tabs.len();
            self.active_tab = self.tabs[next_index];
        }
    }

    pub fn previous_tab(&mut self) {
        if let Some(current_index) = self.tabs.iter().position(|&tab| tab == self.active_tab) {
            let prev_index = if current_index == 0 {
                self.tabs.len() - 1
            } else {
                current_index - 1
            };
            self.active_tab = self.tabs[prev_index];
        }
    }
}

impl Default for TabView {
    fn default() -> Self {
        Self::new()
    }
}