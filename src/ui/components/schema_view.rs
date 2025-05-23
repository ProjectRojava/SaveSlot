//! Schema view component for exploring database structure

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};

use crate::ui::messages::Message;

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub row_count: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub foreign_key: Option<String>,
}

pub struct SchemaView {
    tables: Vec<TableInfo>,
    selected_table: Option<String>,
    loading: bool,
}

impl SchemaView {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            selected_table: None,
            loading: false,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Database Schema").size(18);

        let refresh_button = button(text("Refresh"))
            .on_press(Message::SchemaRefreshRequested)
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

        let toolbar = row![
            title,
            iced::widget::Space::with_width(Length::Fill),
            refresh_button
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center);

        if self.loading {
            let content = column![
                toolbar,
                iced::widget::Rule::horizontal(1),
                container(text("Loading schema...").size(14))
                    .padding(20)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
            ]
            .spacing(10);

            return container(content)
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
                .into();
        }

        if self.tables.is_empty() {
            let content = column![
                toolbar,
                iced::widget::Rule::horizontal(1),
                container(
                    column![
                        text("No schema information available").size(14),
                        text("Connect to a database and refresh to see schema").size(12)
                    ]
                    .spacing(5)
                    .align_x(iced::Alignment::Center)
                )
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
            ]
            .spacing(10);

            return container(content)
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
                .into();
        }

        let tables_list = self.tables
            .iter()
            .fold(column![].spacing(5), |col, table| {
                col.push(self.table_item(table))
            });

        let schema_content = if let Some(selected_table) = &self.selected_table {
            if let Some(table) = self.tables.iter().find(|t| t.name == *selected_table) {
                row![
                    container(
                        column![
                            text("Tables").size(16),
                            scrollable(tables_list)
                        ]
                        .spacing(10)
                    )
                    .padding(10)
                    .width(Length::FillPortion(1))
                    .style(|_theme| iced::widget::container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.98, 0.98, 0.98))),
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.9, 0.9, 0.9),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        shadow: iced::Shadow::default(),
                        text_color: None,
                    }),
                    container(self.table_details(table))
                        .padding(10)
                        .width(Length::FillPortion(2))
                        .style(|_theme| iced::widget::container::Style {
                            background: Some(iced::Background::Color(iced::Color::WHITE)),
                            border: iced::Border {
                                color: iced::Color::from_rgb(0.9, 0.9, 0.9),
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            shadow: iced::Shadow::default(),
                            text_color: None,
                        })
                ]
                .spacing(10)
            } else {
                row![
                    container(
                        column![
                            text("Tables").size(16),
                            scrollable(tables_list)
                        ]
                        .spacing(10)
                    )
                    .padding(10)
                    .width(Length::Fill)
                ]
            }
        } else {
            row![
                container(
                    column![
                        text("Tables").size(16),
                        scrollable(tables_list)
                    ]
                    .spacing(10)
                )
                .padding(10)
                .width(Length::Fill)
                .style(|_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.98, 0.98, 0.98))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.9, 0.9, 0.9),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                    text_color: None,
                })
            ]
        };

        let content = column![
            toolbar,
            iced::widget::Rule::horizontal(1),
            schema_content
        ]
        .spacing(10);

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
    }

    // Fixed lifetime issue by using 'static lifetime
    fn table_item(&self, table: &TableInfo) -> Element<'static, Message> {
        let is_selected = self.selected_table.as_ref() == Some(&table.name);
        
        let table_name = text(table.name.clone()).size(14);
        let column_count = text(format!("{} columns", table.columns.len())).size(12);
        let row_count_text = if let Some(count) = table.row_count {
            text(format!("{} rows", count)).size(12)
        } else {
            text("Unknown rows").size(12)
        };

        let content = column![
            table_name,
            column_count,
            row_count_text
        ]
        .spacing(2);

        button(
            container(content)
                .padding(8)
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(
                        if is_selected {
                            iced::Color::from_rgb(0.9, 0.95, 1.0)
                        } else {
                            iced::Color::WHITE
                        }
                    )),
                    border: iced::Border {
                        color: if is_selected {
                            iced::Color::from_rgb(0.2, 0.6, 1.0)
                        } else {
                            iced::Color::from_rgb(0.9, 0.9, 0.9)
                        },
                        width: if is_selected { 2.0 } else { 1.0 },
                        radius: 4.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                    text_color: None,
                })
        )
        .style(|_theme, _status| iced::widget::button::Style {
            background: None,
            text_color: iced::Color::TRANSPARENT,
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow::default(),
        })
        .on_press(Message::SchemaTableSelected(table.name.clone()))
        .into()
    }

    // Fixed lifetime issue by using 'static lifetime
    fn table_details(&self, table: &TableInfo) -> Element<'static, Message> {
        let title = text(format!("Table: {}", table.name)).size(16);

        let columns_header = row![
            text("Column").size(12),
            text("Type").size(12),
            text("Nullable").size(12),
            text("Key").size(12)
        ]
        .spacing(20);

        let columns_list = table.columns
            .iter()
            .fold(column![].spacing(2), |col, column| {
                col.push(self.column_item(column, &table.name))
            });

        column![
            title,
            iced::widget::Rule::horizontal(1),
            columns_header,
            scrollable(columns_list)
        ]
        .spacing(10)
        .into()
    }

    // Fixed lifetime issue by using 'static lifetime
    fn column_item(&self, column: &ColumnInfo, table_name: &str) -> Element<'static, Message> {
        let key_indicator = if column.primary_key {
            "PK"
        } else if column.foreign_key.is_some() {
            "FK"
        } else {
            ""
        };

        let row_content = row![
            text(column.name.clone()).size(12),
            text(column.data_type.clone()).size(12),
            text(if column.nullable { "Yes" } else { "No" }).size(12),
            text(key_indicator).size(12)
        ]
        .spacing(20);

        button(
            container(row_content)
                .padding(4)
                .style(|_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.99, 0.99, 0.99))),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.95, 0.95, 0.95),
                        width: 1.0,
                        radius: 2.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                    text_color: None,
                })
        )
        .style(|_theme, _status| iced::widget::button::Style {
            background: None,
            text_color: iced::Color::TRANSPARENT,
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 2.0.into(),
            },
            shadow: iced::Shadow::default(),
        })
        .on_press(Message::SchemaColumnSelected(table_name.to_string(), column.name.clone()))
        .into()
    }

    // Public methods for updating state
    pub fn set_tables(&mut self, tables: Vec<TableInfo>) {
        self.tables = tables;
        self.loading = false;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    pub fn select_table(&mut self, table_name: Option<String>) {
        self.selected_table = table_name;
    }

    pub fn clear_schema(&mut self) {
        self.tables.clear();
        self.selected_table = None;
        self.loading = false;
    }

    pub fn get_tables(&self) -> Vec<TableInfo> {
        self.tables.clone()
    }

    // Fixed lifetime issue by returning owned String instead of borrowed &str
    pub fn get_selected_table(&self) -> Option<String> {
        self.selected_table.clone()
    }
}

impl Default for SchemaView {
    fn default() -> Self {
        Self::new()
    }
}