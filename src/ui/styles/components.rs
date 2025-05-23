//! Component-specific styling utilities and presets

use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Shadow, Vector};

use super::colors::ThemeColors;
use super::{radius, shadows};

/// Database-specific component styles
pub mod database {
    use super::*;

    /// Connection status indicator styles
    pub fn connection_status_style(colors: &ThemeColors, is_connected: bool) -> container::Style {
        let (bg_color, border_color) = if is_connected {
            (colors.success_container, colors.success)
        } else {
            (colors.error_container, colors.error)
        };

        container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: radius::SMALL.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(if is_connected {
                colors.on_success_container
            } else {
                colors.on_error_container
            }),
        }
    }

    /// Query editor container style
    pub fn query_editor_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::small(),
            text_color: Some(colors.on_surface),
        }
    }

    /// Results table style
    pub fn results_table_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::small(),
            text_color: Some(colors.on_surface),
        }
    }

    /// Schema visualizer style
    pub fn schema_visualizer_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface_variant)),
            border: Border {
                color: colors.outline,
                width: 1.0,
                radius: radius::LARGE.into(),
            },
            shadow: shadows::medium(),
            text_color: Some(colors.on_surface_variant),
        }
    }

    /// SQL syntax highlighting colors
    pub struct SqlSyntaxColors {
        pub keyword: Color,
        pub string: Color,
        pub comment: Color,
        pub number: Color,
        pub operator: Color,
        pub function: Color,
        pub table_name: Color,
        pub column_name: Color,
    }

    pub fn sql_syntax_colors(colors: &ThemeColors) -> SqlSyntaxColors {
        SqlSyntaxColors {
            keyword: colors.syntax_keyword,
            string: colors.syntax_string,
            comment: colors.syntax_comment,
            number: colors.syntax_number,
            operator: colors.primary,
            function: Color::from_rgb(0.6, 0.4, 0.8), // Purple
            table_name: Color::from_rgb(0.8, 0.6, 0.2), // Orange
            column_name: colors.on_surface,
        }
    }
}

/// Navigation component styles
pub mod navigation {
    use super::*;

    /// Sidebar navigation style
    pub fn sidebar_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface_variant)),
            border: Border {
                color: colors.outline_variant,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(2.0, 0.0),
                blur_radius: 4.0,
            },
            text_color: Some(colors.on_surface_variant),
        }
    }

    /// Navigation item style
    pub fn nav_item_style(colors: &ThemeColors, is_active: bool) -> button::Style {
        if is_active {
            button::Style {
                background: Some(Background::Color(colors.primary_container)),
                text_color: colors.on_primary_container,
                border: Border {
                    color: colors.primary,
                    width: 1.0,
                    radius: radius::MEDIUM.into(),
                },
                shadow: Shadow::default(),
            }
        } else {
            button::Style {
                background: None,
                text_color: colors.on_surface_variant,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: radius::MEDIUM.into(),
                },
                shadow: Shadow::default(),
            }
        }
    }

    /// Breadcrumb style
    pub fn breadcrumb_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: radius::SMALL.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(colors.on_surface),
        }
    }
}

/// Dialog and modal styles
pub mod dialogs {
    use super::*;

    /// Modal overlay style
    pub fn modal_overlay_style() -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
            border: Border::default(),
            shadow: Shadow::default(),
            text_color: None,
        }
    }

    /// Dialog container style
    pub fn dialog_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline,
                width: 1.0,
                radius: radius::LARGE.into(),
            },
            shadow: shadows::large(),
            text_color: Some(colors.on_surface),
        }
    }

    /// Alert dialog style
    pub fn alert_style(colors: &ThemeColors, alert_type: AlertType) -> container::Style {
        let (bg_color, border_color, text_color) = match alert_type {
            AlertType::Info => (
                colors.primary_container,
                colors.primary,
                colors.on_primary_container,
            ),
            AlertType::Success => (
                colors.success_container,
                colors.success,
                colors.on_success_container,
            ),
            AlertType::Warning => (
                colors.warning_container,
                colors.warning,
                colors.on_warning_container,
            ),
            AlertType::Error => (
                colors.error_container,
                colors.error,
                colors.on_error_container,
            ),
        };

        container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::medium(),
            text_color: Some(text_color),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AlertType {
        Info,
        Success,
        Warning,
        Error,
    }
}

/// Form component styles
pub mod forms {
    use super::*;

    /// Form container style
    pub fn form_container_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::small(),
            text_color: Some(colors.on_surface),
        }
    }

    /// Form field style
    pub fn form_field_style(colors: &ThemeColors, has_error: bool) -> text_input::Style {
        text_input::Style {
            background: Background::Color(colors.surface),
            border: Border {
                color: if has_error {
                    colors.error
                } else {
                    colors.outline
                },
                width: 1.0,
                radius: radius::SMALL.into(),
            },
            icon: colors.on_surface_variant,
            placeholder: colors.on_surface_variant,
            selection: colors.primary,
            value: colors.on_surface,
        }
    }

    /// Field label style
    pub fn field_label_color(colors: &ThemeColors, has_error: bool) -> Color {
        if has_error {
            colors.error
        } else {
            colors.on_surface
        }
    }

    /// Help text color
    pub fn help_text_color(colors: &ThemeColors) -> Color {
        colors.on_surface_variant
    }

    /// Error text color
    pub fn error_text_color(colors: &ThemeColors) -> Color {
        colors.error
    }
}

/// Toolbar and action bar styles
pub mod toolbars {
    use super::*;

    /// Main toolbar style
    pub fn main_toolbar_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
            text_color: Some(colors.on_surface),
        }
    }

    /// Toolbar button style
    pub fn toolbar_button_style(colors: &ThemeColors) -> button::Style {
        button::Style {
            background: None,
            text_color: colors.on_surface,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius::SMALL.into(),
            },
            shadow: Shadow::default(),
        }
    }

    /// Action button style
    pub fn action_button_style(colors: &ThemeColors, action_type: ActionType) -> button::Style {
        let (bg_color, text_color) = match action_type {
            ActionType::Primary => (colors.primary, colors.on_primary),
            ActionType::Secondary => (colors.secondary_container, colors.on_secondary_container),
            ActionType::Destructive => (colors.error, colors.on_error),
            ActionType::Constructive => (colors.success, colors.on_success),
        };

        button::Style {
            background: Some(Background::Color(bg_color)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::small(),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ActionType {
        Primary,
        Secondary,
        Destructive,
        Constructive,
    }
}

/// Status and notification styles
pub mod status {
    use super::*;

    /// Status bar style
    pub fn status_bar_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface_variant)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(colors.on_surface_variant),
        }
    }

    /// Notification style
    pub fn notification_style(
        colors: &ThemeColors,
        notification_type: NotificationType,
    ) -> container::Style {
        let (bg_color, border_color, text_color) = match notification_type {
            NotificationType::Info => (
                colors.primary_container,
                colors.primary,
                colors.on_primary_container,
            ),
            NotificationType::Success => (
                colors.success_container,
                colors.success,
                colors.on_success_container,
            ),
            NotificationType::Warning => (
                colors.warning_container,
                colors.warning,
                colors.on_warning_container,
            ),
            NotificationType::Error => (
                colors.error_container,
                colors.error,
                colors.on_error_container,
            ),
        };

        container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::medium(),
            text_color: Some(text_color),
        }
    }

    /// Progress indicator style
    pub fn progress_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface_variant)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: radius::SMALL.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(colors.on_surface_variant),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum NotificationType {
        Info,
        Success,
        Warning,
        Error,
    }
}

/// Data visualization styles
pub mod visualization {
    use super::*;

    /// Chart container style
    pub fn chart_container_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::small(),
            text_color: Some(colors.on_surface),
        }
    }

    /// Data grid style
    pub fn data_grid_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline,
                width: 1.0,
                radius: radius::SMALL.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(colors.on_surface),
        }
    }

    /// Grid header style
    pub fn grid_header_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface_variant)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(colors.on_surface_variant),
        }
    }

    /// Grid row style
    pub fn grid_row_style(
        colors: &ThemeColors,
        is_selected: bool,
        is_even: bool,
    ) -> container::Style {
        let bg_color = if is_selected {
            colors.primary_container
        } else if is_even {
            colors.surface
        } else {
            Color {
                a: 0.5,
                ..colors.surface_variant
            }
        };

        let text_color = if is_selected {
            colors.on_primary_container
        } else {
            colors.on_surface
        };

        container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                color: colors.outline_variant,
                width: 0.5,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(text_color),
        }
    }
}

/// AI Assistant specific styles
pub mod ai_assistant {
    use super::*;

    /// Chat container style
    pub fn chat_container_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.surface)),
            border: Border {
                color: colors.outline_variant,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: shadows::small(),
            text_color: Some(colors.on_surface),
        }
    }

    /// User message style
    pub fn user_message_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.primary_container)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(colors.on_primary_container),
        }
    }

    /// AI message style
    pub fn ai_message_style(colors: &ThemeColors) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors.secondary_container)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(colors.on_secondary_container),
        }
    }

    /// Suggestion button style
    pub fn suggestion_button_style(colors: &ThemeColors) -> button::Style {
        button::Style {
            background: Some(Background::Color(colors.surface_variant)),
            text_color: colors.primary,
            border: Border {
                color: colors.primary,
                width: 1.0,
                radius: radius::MEDIUM.into(),
            },
            shadow: Shadow::default(),
        }
    }
}

/// Utility functions for creating component styles
pub mod utils {
    use super::*;

    /// Create a hover effect for containers
    pub fn with_hover_effect(
        mut style: container::Style,
        _colors: &ThemeColors,
    ) -> container::Style {
        if let Some(Background::Color(color)) = style.background {
            style.background = Some(Background::Color(Color {
                a: (color.a * 1.1).min(1.0),
                ..color
            }));
        }
        style.shadow = shadows::medium();
        style
    }

    /// Create a pressed effect for buttons
    pub fn with_pressed_effect(mut style: button::Style) -> button::Style {
        if let Some(Background::Color(color)) = style.background {
            style.background = Some(Background::Color(Color {
                a: (color.a * 0.8).max(0.1),
                ..color
            }));
        }
        style.shadow = Shadow::default();
        style
    }

    /// Create a disabled effect for any component
    pub fn with_disabled_effect(
        mut style: container::Style,
        colors: &ThemeColors,
    ) -> container::Style {
        style.background = Some(Background::Color(colors.surface_variant));
        style.text_color = Some(colors.on_surface_variant);
        style.shadow = Shadow::default();
        style
    }

    /// Create a focus ring effect
    pub fn with_focus_ring(mut border: Border, colors: &ThemeColors) -> Border {
        border.color = colors.primary;
        border.width = 2.0;
        border
    }
}

/// Predefined component style combinations
pub mod presets {
    use super::*;

    /// Database connection card preset
    pub fn connection_card(colors: &ThemeColors, is_active: bool) -> container::Style {
        let mut style = database::query_editor_style(colors);
        if is_active {
            style.border.color = colors.primary;
            style.border.width = 2.0;
            style.shadow = shadows::medium();
        }
        style
    }

    /// Query result table preset
    pub fn result_table(colors: &ThemeColors) -> container::Style {
        database::results_table_style(colors)
    }

    /// AI chat message preset
    pub fn chat_message(colors: &ThemeColors, is_user: bool) -> container::Style {
        if is_user {
            ai_assistant::user_message_style(colors)
        } else {
            ai_assistant::ai_message_style(colors)
        }
    }

    /// Form field with validation preset
    pub fn validated_field(
        colors: &ThemeColors,
        validation_state: ValidationState,
    ) -> text_input::Style {
        match validation_state {
            ValidationState::Valid => forms::form_field_style(colors, false),
            ValidationState::Invalid => forms::form_field_style(colors, true),
            ValidationState::Pending => {
                let mut style = forms::form_field_style(colors, false);
                style.border.color = colors.warning;
                style
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ValidationState {
        Valid,
        Invalid,
        Pending,
    }
}
