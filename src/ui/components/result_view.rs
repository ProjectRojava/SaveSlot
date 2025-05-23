//! Result view component for displaying query results

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};

use crate::ui::messages::Message;

pub struct ResultView {
    results: Option<Vec<Vec<String>>>,
    columns: Vec<String>,
    row_count: usize,
    execution_time: Option<std::time::Duration>,
    current_page: usize,
    rows_per_page: usize,
}

impl ResultView {
    pub fn new() -> Self {
        Self {
            results: None,
            columns: Vec::new(),
            row_count: 0,
            execution_time: None,
            current_page: 0,
            rows_per_page: 100,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Query Results").size(18);

        if let Some(results) = &self.results {
            if results.is_empty() {
                return container(
                    column![
                        title,
                        text("No results to display").size(14)
                    ]
                    .spacing(10)
                    .align_x(iced::Alignment::Center)
                )
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into();
            }

            let header_row = if !results.is_empty() {
                results[0]
                    .iter()
                    .fold(row![].spacing(10), |row_widget, header| {
                        row_widget.push(
                            container(text(header).size(14))
                                .padding(8)
                                .style(|_theme| iced::widget::container::Style {
                                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9))),
                                    border: iced::Border {
                                        color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                                        width: 1.0,
                                        radius: 0.0.into(),
                                    },
                                    shadow: iced::Shadow::default(),
                                    text_color: None,
                                })
                                .width(Length::Fill)
                        )
                    })
            } else {
                row![]
            };

            let data_rows = results
                .iter()
                .skip(1) // Skip header row
                .skip(self.current_page * self.rows_per_page)
                .take(self.rows_per_page)
                .fold(column![].spacing(1), |col, row_data| {
                    let data_row = row_data
                        .iter()
                        .fold(row![].spacing(10), |row_widget, cell| {
                            row_widget.push(
                                container(text(cell).size(12))
                                    .padding(8)
                                    .style(|_theme| iced::widget::container::Style {
                                        background: Some(iced::Background::Color(iced::Color::WHITE)),
                                        border: iced::Border {
                                            color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                                            width: 1.0,
                                            radius: 0.0.into(),
                                        },
                                        shadow: iced::Shadow::default(),
                                        text_color: None,
                                    })
                                    .width(Length::Fill)
                            )
                        });
                    col.push(data_row)
                });

            let table = column![header_row, data_rows].spacing(0);

            let stats_text = if let Some(duration) = self.execution_time {
                format!(
                    "Showing {} rows (Execution time: {:.2}ms)",
                    results.len().saturating_sub(1),
                    duration.as_millis()
                )
            } else {
                format!("Showing {} rows", results.len().saturating_sub(1))
            };

            let stats_bar = container(text(stats_text).size(12))
                .padding(8)
                .style(|_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.95, 0.95, 0.95))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                    text_color: None,
                });

            let export_button = button(text("Export"))
                .on_press(Message::ResultsExported)
                .style(|_theme, _status| iced::widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.6, 1.0))),
                    text_color: iced::Color::WHITE,
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                });

            let clear_button = button(text("Clear"))
                .on_press(Message::ResultsCleared)
                .style(|_theme, _status| iced::widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9))),
                    text_color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                });

            let toolbar = row![
                title,
                iced::widget::Space::with_width(Length::Fill),
                export_button,
                clear_button
            ]
            .spacing(10)
            .align_y(iced::Alignment::Center);

            let content = column![
                toolbar,
                iced::widget::Rule::horizontal(1),
                scrollable(table),
                stats_bar
            ]
            .spacing(5);

            container(content)
                .padding(10)
                .style(|_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::WHITE)),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.9, 0.9, 0.9),
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                    text_color: None,
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            container(
                column![
                    title,
                    text("No query results yet").size(14),
                    text("Execute a query to see results here").size(12)
                ]
                .spacing(10)
                .align_x(iced::Alignment::Center)
            )
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        }
    }

    // Public methods for updating state
    pub fn set_results(&mut self, results: Vec<Vec<String>>) {
        self.results = Some(results.clone());
        if !results.is_empty() {
            self.columns = results[0].clone();
            self.row_count = results.len().saturating_sub(1); // Subtract header row
        }
        self.current_page = 0;
    }

    pub fn clear_results(&mut self) {
        self.results = None;
        self.columns.clear();
        self.row_count = 0;
        self.execution_time = None;
        self.current_page = 0;
    }

    pub fn set_execution_time(&mut self, duration: std::time::Duration) {
        self.execution_time = Some(duration);
    }

    pub fn next_page(&mut self) {
        if let Some(results) = &self.results {
            let max_page = (results.len().saturating_sub(1)) / self.rows_per_page;
            if self.current_page < max_page {
                self.current_page += 1;
            }
        }
    }

    pub fn previous_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    pub fn get_results(&self) -> Option<&Vec<Vec<String>>> {
        self.results.as_ref()
    }
}

impl Default for ResultView {
    fn default() -> Self {
        Self::new()
    }
}