//! Query editor component for writing and executing SQL queries

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Length};

use crate::ui::messages::{Message, QueryEditorState, QueryHistoryItem};

pub struct QueryEditor {
    state: QueryEditorState,
    query_history: Vec<QueryHistoryItem>,
    show_history: bool,
}

impl QueryEditor {
    pub fn new() -> Self {
        Self {
            state: QueryEditorState::default(),
            query_history: Vec::new(),
            show_history: false,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("SQL Query Editor").size(18);

        // Toolbar buttons
        let execute_button = button(text("Execute"))
            .on_press(Message::QueryExecuteRequested)
            .style(|_theme, _status| self.primary_button_style(_theme, _status));

        let save_button = button(text("Save"))
            .on_press(Message::QuerySaveRequested)
            .style(|_theme, _status| self.secondary_button_style(_theme, _status));

        let load_button = button(text("Load"))
            .on_press(Message::QueryLoadRequested)
            .style(|_theme, _status| self.secondary_button_style(_theme, _status));

        let clear_button = button(text("Clear"))
            .on_press(Message::QueryCleared)
            .style(|_theme, _status| self.secondary_button_style(_theme, _status));

        let format_button = button(text("Format"))
            .on_press(Message::QueryFormatRequested)
            .style(|_theme, _status| self.secondary_button_style(_theme, _status));

        let explain_button = button(text("Explain"))
            .on_press(Message::QueryExplainRequested)
            .style(|_theme, _status| self.secondary_button_style(_theme, _status));

        let history_button = button(text("History"))
            .on_press(Message::QueryHistoryRequested)
            .style(|_theme, _status| self.secondary_button_style(_theme, _status));

        let toolbar = row![
            title,
            iced::widget::Space::with_width(Length::Fill),
            execute_button,
            save_button,
            load_button,
            clear_button,
            format_button,
            explain_button,
            history_button
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center);

        // Query input area
        let query_input = text_input("Enter your SQL query here...", &self.state.current_query)
            .on_input(Message::QueryTextChanged)
            .style(|_theme, _status| self.text_input_style(_theme, _status))
            .size(14);

        // Status bar
        let status_text = if self.state.is_executing {
            "Executing query...".to_string()
        } else if let Some(duration) = self.state.last_execution_time {
            format!("Last execution: {:.2}ms", duration.as_millis())
        } else {
            "Ready".to_string()
        };

        let status_bar = container(text(status_text).size(12))
            .padding(5)
            .style(|_theme| self.status_bar_style(_theme));

        // Main content
        let mut content_column = column![
            toolbar,
            iced::widget::Rule::horizontal(1),
            container(query_input)
                .padding(10)
                .height(Length::Fixed(200.0))
                .style(|_theme| self.editor_container_style(_theme)),
            status_bar
        ]
        .spacing(5);

        // Add history panel if visible
        if self.show_history {
            content_column = content_column.push(self.history_panel());
        }

        container(content_column)
            .padding(10)
            .style(|_theme| self.main_container_style(_theme))
            .into()
    }

    fn history_panel(&self) -> Element<Message> {
        let title = text("Query History").size(16);

        let history_list = if self.query_history.is_empty() {
            column![text("No query history").size(12)].align_x(iced::Alignment::Center)
        } else {
            self.query_history
                .iter()
                .fold(column![].spacing(2), |col, item| {
                    col.push(self.history_item(item))
                })
        };

        let content = column![title, iced::widget::Rule::horizontal(1), history_list].spacing(5);

        container(content)
            .padding(10)
            .height(Length::Fixed(150.0))
            .style(|_theme| self.history_container_style(_theme))
            .into()
    }

    fn history_item(&self, item: &QueryHistoryItem) -> Element<Message> {
        let query_preview = if item.query.len() > 50 {
            format!("{}...", &item.query[..47])
        } else {
            item.query.clone()
        };

        let time_text = item.executed_at.format("%H:%M:%S").to_string();
        let status_icon = if item.success { "✓" } else { "✗" };

        let row_content = row![
            text(status_icon).size(12),
            text(query_preview).size(12),
            iced::widget::Space::with_width(Length::Fill),
            text(time_text).size(10)
        ]
        .spacing(5)
        .align_y(iced::Alignment::Center);

        button(
            container(row_content)
                .padding(5)
                .style(|_theme| self.history_item_style(_theme)),
        )
        .style(|_theme, _status| self.history_button_style(_theme, _status))
        .on_press(Message::QueryFromHistorySelected(item.id))
        .into()
    }

    // Style methods
    fn primary_button_style(
        &self,
        _theme: &iced::Theme,
        _status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.2, 0.6, 1.0,
            ))),
            text_color: iced::Color::WHITE,
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow::default(),
        }
    }

    fn secondary_button_style(
        &self,
        _theme: &iced::Theme,
        _status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.9, 0.9, 0.9,
            ))),
            text_color: iced::Color::from_rgb(0.2, 0.2, 0.2),
            border: iced::Border {
                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow::default(),
        }
    }

    fn text_input_style(
        &self,
        _theme: &iced::Theme,
        _status: iced::widget::text_input::Status,
    ) -> iced::widget::text_input::Style {
        iced::widget::text_input::Style {
            background: iced::Background::Color(iced::Color::WHITE),
            border: iced::Border {
                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon: iced::Color::from_rgb(0.5, 0.5, 0.5),
            placeholder: iced::Color::from_rgb(0.7, 0.7, 0.7),
            value: iced::Color::from_rgb(0.1, 0.1, 0.1),
            selection: iced::Color::from_rgb(0.2, 0.6, 1.0),
        }
    }

    fn main_container_style(&self, _theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::Border {
                color: iced::Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: iced::Shadow::default(),
            text_color: None,
        }
    }

    fn editor_container_style(&self, _theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.98, 0.98, 0.98,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow::default(),
            text_color: None,
        }
    }

    fn status_bar_style(&self, _theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.95, 0.95, 0.95,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow::default(),
            text_color: None,
        }
    }

    fn history_container_style(&self, _theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.97, 0.97, 0.97,
            ))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow::default(),
            text_color: None,
        }
    }

    fn history_item_style(&self, _theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::Border {
                color: iced::Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                radius: 2.0.into(),
            },
            shadow: iced::Shadow::default(),
            text_color: None,
        }
    }

    fn history_button_style(
        &self,
        _theme: &iced::Theme,
        _status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        iced::widget::button::Style {
            background: None,
            text_color: iced::Color::TRANSPARENT,
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 2.0.into(),
            },
            shadow: iced::Shadow::default(),
        }
    }

    // Public methods for updating state
    pub fn update_query(&mut self, query: String) {
        self.state.current_query = query;
    }

    pub fn set_executing(&mut self, executing: bool) {
        self.state.is_executing = executing;
    }

    pub fn set_execution_time(&mut self, duration: std::time::Duration) {
        self.state.last_execution_time = Some(duration);
    }

    pub fn add_to_history(&mut self, item: QueryHistoryItem) {
        self.query_history.push(item);
        // Keep only last 50 items
        if self.query_history.len() > 50 {
            self.query_history.remove(0);
        }
    }

    pub fn toggle_history(&mut self) {
        self.show_history = !self.show_history;
    }

    pub fn clear_query(&mut self) {
        self.state.current_query.clear();
    }

    pub fn get_current_query(&self) -> &str {
        &self.state.current_query
    }
}

impl Default for QueryEditor {
    fn default() -> Self {
        Self::new()
    }
}
