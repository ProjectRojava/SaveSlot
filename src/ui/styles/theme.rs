//! Theme management for Rojava SaveSlot
//!
//! Provides custom _theme implementations and _theme switching functionality

use iced::{Background, Border, Color, Shadow, Theme, Vector};

use super::colors::{DARK_THEME, HIGH_CONTRAST_THEME, LIGHT_THEME, ThemeColors};

/// Custom _theme implementation for Rojava SaveSlot
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RojavaTheme {
    #[default]
    Light,
    Dark,
    HighContrast,
}

impl RojavaTheme {
    /// Get all available themes
    pub fn all() -> &'static [RojavaTheme] {
        &[
            RojavaTheme::Light,
            RojavaTheme::Dark,
            RojavaTheme::HighContrast,
        ]
    }

    /// Get the color palette for this _theme
    pub fn colors(self) -> ThemeColors {
        match self {
            RojavaTheme::Light => LIGHT_THEME,
            RojavaTheme::Dark => DARK_THEME,
            RojavaTheme::HighContrast => HIGH_CONTRAST_THEME,
        }
    }

    /// Convert to Iced's built-in _theme
    pub fn to_iced_theme(self) -> Theme {
        match self {
            RojavaTheme::Light => Theme::Light,
            RojavaTheme::Dark => Theme::Dark,
            RojavaTheme::HighContrast => Theme::Dark,
        }
    }
}

impl std::fmt::Display for RojavaTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RojavaTheme::Light => write!(f, "Light"),
            RojavaTheme::Dark => write!(f, "Dark"),
            RojavaTheme::HighContrast => write!(f, "High Contrast"),
        }
    }
}

/// Button style variants for Iced 0.13.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    Danger,
    Success,
    Ghost,
    Icon,
}

/// Container style variants for Iced 0.13.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContainerStyle {
    #[default]
    Default,
    Card,
    Sidebar,
    Panel,
    Transparent,
}

/// Text input style variants for Iced 0.13.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextInputStyle {
    #[default]
    Default,
    Code,
}

/// Theme manager for handling _theme switching and persistence
pub struct ThemeManager {
    current_theme: RojavaTheme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: RojavaTheme::default(),
        }
    }

    pub fn current_theme(&self) -> RojavaTheme {
        self.current_theme
    }

    pub fn set_theme(&mut self, _theme: RojavaTheme) {
        self.current_theme = _theme;
    }

    pub fn toggle_theme(&mut self) {
        self.current_theme = match self.current_theme {
            RojavaTheme::Light => RojavaTheme::Dark,
            RojavaTheme::Dark => RojavaTheme::HighContrast,
            RojavaTheme::HighContrast => RojavaTheme::Light,
        };
    }

    pub fn is_dark(&self) -> bool {
        matches!(
            self.current_theme,
            RojavaTheme::Dark | RojavaTheme::HighContrast
        )
    }

    /// Load _theme from configuration (placeholder for future implementation)
    pub fn load_from_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement loading from config file
        Ok(())
    }

    /// Save _theme to configuration (placeholder for future implementation)
    pub fn save_to_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement saving to config file
        Ok(())
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for creating themed elements with Iced 0.13.1
pub mod themed {
    use super::*;
    use iced::Element;
    use iced::widget::{Button, Container, TextInput};

    /// Create a primary button with _theme
    pub fn primary_button<'a, Message>(
        content: impl Into<Element<'a, Message>>,
    ) -> Button<'a, Message> {
        Button::new(content).style(|_theme: &Theme, status| {
            let colors = RojavaTheme::Light.colors();
            match status {
                iced::widget::button::Status::Active => iced::widget::button::Style {
                    background: Some(Background::Color(colors.primary)),
                    text_color: colors.on_primary,
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
                },
                iced::widget::button::Status::Hovered => iced::widget::button::Style {
                    background: Some(Background::Color(Color {
                        a: 0.8,
                        ..colors.primary
                    })),
                    text_color: colors.on_primary,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: Vector::new(0.0, 4.0),
                        blur_radius: 8.0,
                    },
                },
                iced::widget::button::Status::Pressed => iced::widget::button::Style {
                    background: Some(Background::Color(Color {
                        a: 0.6,
                        ..colors.primary
                    })),
                    text_color: colors.on_primary,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow::default(),
                },
                iced::widget::button::Status::Disabled => iced::widget::button::Style {
                    background: Some(Background::Color(colors.surface_variant)),
                    text_color: colors.on_surface_variant,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow::default(),
                },
            }
        })
    }

    /// Create a secondary button with _theme
    pub fn secondary_button<'a, Message>(
        content: impl Into<Element<'a, Message>>,
    ) -> Button<'a, Message> {
        Button::new(content).style(|_theme: &Theme, status| {
            let colors = RojavaTheme::Light.colors();
            match status {
                iced::widget::button::Status::Active => iced::widget::button::Style {
                    background: Some(Background::Color(colors.secondary_container)),
                    text_color: colors.on_secondary_container,
                    border: Border {
                        color: colors.outline,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow::default(),
                },
                _ => iced::widget::button::Style {
                    background: Some(Background::Color(colors.secondary_container)),
                    text_color: colors.on_secondary_container,
                    border: Border {
                        color: colors.outline,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow::default(),
                },
            }
        })
    }

    /// Create a card container with _theme
    pub fn card<'a, Message>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
        Container::new(content).style(|_theme: &Theme| {
            let colors = RojavaTheme::Light.colors();
            iced::widget::container::Style {
                background: Some(Background::Color(colors.surface)),
                border: Border {
                    color: colors.outline_variant,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                text_color: Some(colors.on_surface),
            }
        })
    }

    /// Create a panel container with _theme
    pub fn panel<'a, Message>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
        Container::new(content).style(|_theme: &Theme| {
            let colors = RojavaTheme::Light.colors();
            iced::widget::container::Style {
                background: Some(Background::Color(colors.surface_variant)),
                border: Border {
                    color: colors.outline_variant,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: Shadow::default(),
                text_color: Some(colors.on_surface_variant),
            }
        })
    }

    /// Create a code text input with _theme
    pub fn code_input<'a, Message>(
        placeholder: &str,
        value: &str,
        on_change: impl Fn(String) -> Message + 'a,
    ) -> TextInput<'a, Message>
    where
        Message: Clone,
    {
        TextInput::new(placeholder, value)
            .on_input(on_change)
            .style(|_theme: &Theme, status| {
                let colors = RojavaTheme::Light.colors();
                match status {
                    iced::widget::text_input::Status::Active => iced::widget::text_input::Style {
                        background: Background::Color(colors.surface),
                        border: Border {
                            color: colors.outline,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        icon: colors.on_surface_variant,
                        placeholder: colors.on_surface_variant,
                        value: colors.on_surface,
                        selection: colors.primary,
                    },
                    iced::widget::text_input::Status::Focused => iced::widget::text_input::Style {
                        background: Background::Color(colors.surface),
                        border: Border {
                            color: colors.primary,
                            width: 2.0,
                            radius: 4.0.into(),
                        },
                        icon: colors.on_surface,
                        placeholder: colors.on_surface_variant,
                        value: colors.on_surface,
                        selection: colors.primary,
                    },
                    iced::widget::text_input::Status::Disabled => iced::widget::text_input::Style {
                        background: Background::Color(colors.surface_variant),
                        border: Border {
                            color: colors.outline_variant,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        icon: colors.on_surface_variant,
                        placeholder: colors.on_surface_variant,
                        value: colors.on_surface_variant,
                        selection: colors.outline,
                    },
                    iced::widget::text_input::Status::Hovered => iced::widget::text_input::Style {
                        background: Background::Color(colors.surface),
                        border: Border {
                            color: colors.primary,
                            width: 2.0,
                            radius: 4.0.into(),
                        },
                        icon: colors.on_surface,
                        placeholder: colors.on_surface_variant,
                        value: colors.on_surface,
                        selection: colors.primary,
                    },
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let _theme = RojavaTheme::Light;
        assert_eq!(_theme, RojavaTheme::Light);
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        assert_eq!(manager.current_theme(), RojavaTheme::Light);

        manager.set_theme(RojavaTheme::Dark);
        assert_eq!(manager.current_theme(), RojavaTheme::Dark);
        assert!(manager.is_dark());
    }

    #[test]
    fn test_theme_toggle() {
        let mut manager = ThemeManager::new();

        manager.toggle_theme();
        assert_eq!(manager.current_theme(), RojavaTheme::Dark);

        manager.toggle_theme();
        assert_eq!(manager.current_theme(), RojavaTheme::HighContrast);

        manager.toggle_theme();
        assert_eq!(manager.current_theme(), RojavaTheme::Light);
    }

    #[test]
    fn test_theme_colors() {
        let light_colors = RojavaTheme::Light.colors();
        let dark_colors = RojavaTheme::Dark.colors();

        // Light _theme should have light background
        assert!(light_colors.background.r > 0.5);

        // Dark _theme should have dark background
        assert!(dark_colors.background.r < 0.5);
    }
}
