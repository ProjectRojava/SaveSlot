//! Styling module for Rojava SaveSlot UI
//!
//! Contains theme definitions, color schemes, and component styles for Iced

pub mod colors;
pub mod components;
pub mod theme;
pub mod typography;

// Re-export commonly used items
pub use colors::*;
pub use theme::*;
pub use typography::*;

use iced::Theme;

/// Application theme configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Light,
    Dark,
    HighContrast,
}

impl AppTheme {
    pub fn to_iced_theme(self) -> Theme {
        match self {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
            AppTheme::HighContrast => Theme::Dark, // We'll customize this later
        }
    }

    pub fn all() -> &'static [AppTheme] {
        &[AppTheme::Light, AppTheme::Dark, AppTheme::HighContrast]
    }
}

impl std::fmt::Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppTheme::Light => write!(f, "Light"),
            AppTheme::Dark => write!(f, "Dark"),
            AppTheme::HighContrast => write!(f, "High Contrast"),
        }
    }
}

/// Get the current application theme colors
pub fn get_theme_colors(theme: AppTheme) -> ThemeColors {
    match theme {
        AppTheme::Light => LIGHT_THEME,
        AppTheme::Dark => DARK_THEME,
        AppTheme::HighContrast => HIGH_CONTRAST_THEME,
    }
}

/// Common spacing values used throughout the application
pub mod spacing {
    pub const NONE: f32 = 0.0;
    pub const TINY: f32 = 2.0;
    pub const SMALL: f32 = 4.0;
    pub const MEDIUM: f32 = 8.0;
    pub const LARGE: f32 = 16.0;
    pub const EXTRA_LARGE: f32 = 24.0;
    pub const HUGE: f32 = 32.0;
}

/// Common border radius values
pub mod radius {
    pub const NONE: f32 = 0.0;
    pub const SMALL: f32 = 2.0;
    pub const MEDIUM: f32 = 4.0;
    pub const LARGE: f32 = 8.0;
    pub const ROUND: f32 = 50.0;
}

/// Common shadow configurations
pub mod shadows {
    use iced::{Color, Shadow, Vector};

    pub fn small() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        }
    }

    pub fn medium() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        }
    }

    pub fn large() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        }
    }
}

/// Animation durations in milliseconds
pub mod animations {
    pub const FAST: u64 = 150;
    pub const NORMAL: u64 = 300;
    pub const SLOW: u64 = 500;
}
