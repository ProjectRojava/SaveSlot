//! Color definitions for different themes

use iced::Color;

/// Theme color palette
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    // Background colors
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,

    // Text colors
    pub on_background: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,

    // Primary colors
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,

    // Secondary colors
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,

    // Error colors
    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,

    // Success colors
    pub success: Color,
    pub on_success: Color,
    pub success_container: Color,
    pub on_success_container: Color,

    // Warning colors
    pub warning: Color,
    pub on_warning: Color,
    pub warning_container: Color,
    pub on_warning_container: Color,

    // Border and outline colors
    pub outline: Color,
    pub outline_variant: Color,

    // Special colors for database UI
    pub connection_active: Color,
    pub connection_inactive: Color,
    pub query_success: Color,
    pub query_error: Color,
    pub syntax_keyword: Color,
    pub syntax_string: Color,
    pub syntax_comment: Color,
    pub syntax_number: Color,
}

/// Light theme color palette
pub const LIGHT_THEME: ThemeColors = ThemeColors {
    // Background colors
    background: Color::from_rgb(1.0, 1.0, 1.0),
    surface: Color::from_rgb(0.98, 0.98, 0.98),
    surface_variant: Color::from_rgb(0.95, 0.95, 0.95),

    // Text colors
    on_background: Color::from_rgb(0.1, 0.1, 0.1),
    on_surface: Color::from_rgb(0.2, 0.2, 0.2),
    on_surface_variant: Color::from_rgb(0.4, 0.4, 0.4),

    // Primary colors (Blue theme)
    primary: Color::from_rgb(0.2, 0.6, 1.0),
    on_primary: Color::from_rgb(1.0, 1.0, 1.0),
    primary_container: Color::from_rgb(0.85, 0.93, 1.0),
    on_primary_container: Color::from_rgb(0.0, 0.2, 0.4),

    // Secondary colors
    secondary: Color::from_rgb(0.4, 0.4, 0.4),
    on_secondary: Color::from_rgb(1.0, 1.0, 1.0),
    secondary_container: Color::from_rgb(0.9, 0.9, 0.9),
    on_secondary_container: Color::from_rgb(0.2, 0.2, 0.2),

    // Error colors
    error: Color::from_rgb(0.9, 0.2, 0.2),
    on_error: Color::from_rgb(1.0, 1.0, 1.0),
    error_container: Color::from_rgb(1.0, 0.9, 0.9),
    on_error_container: Color::from_rgb(0.4, 0.0, 0.0),

    // Success colors
    success: Color::from_rgb(0.2, 0.7, 0.3),
    on_success: Color::from_rgb(1.0, 1.0, 1.0),
    success_container: Color::from_rgb(0.9, 1.0, 0.9),
    on_success_container: Color::from_rgb(0.0, 0.3, 0.1),

    // Warning colors
    warning: Color::from_rgb(1.0, 0.6, 0.0),
    on_warning: Color::from_rgb(1.0, 1.0, 1.0),
    warning_container: Color::from_rgb(1.0, 0.95, 0.85),
    on_warning_container: Color::from_rgb(0.4, 0.2, 0.0),

    // Border colors
    outline: Color::from_rgb(0.8, 0.8, 0.8),
    outline_variant: Color::from_rgb(0.9, 0.9, 0.9),

    // Database-specific colors
    connection_active: Color::from_rgb(0.2, 0.8, 0.2),
    connection_inactive: Color::from_rgb(0.6, 0.6, 0.6),
    query_success: Color::from_rgb(0.2, 0.7, 0.3),
    query_error: Color::from_rgb(0.9, 0.2, 0.2),
    syntax_keyword: Color::from_rgb(0.0, 0.0, 0.8),
    syntax_string: Color::from_rgb(0.0, 0.6, 0.0),
    syntax_comment: Color::from_rgb(0.6, 0.6, 0.6),
    syntax_number: Color::from_rgb(0.8, 0.4, 0.0),
};

/// Dark theme color palette
pub const DARK_THEME: ThemeColors = ThemeColors {
    // Background colors
    background: Color::from_rgb(0.1, 0.1, 0.1),
    surface: Color::from_rgb(0.15, 0.15, 0.15),
    surface_variant: Color::from_rgb(0.2, 0.2, 0.2),

    // Text colors
    on_background: Color::from_rgb(0.9, 0.9, 0.9),
    on_surface: Color::from_rgb(0.8, 0.8, 0.8),
    on_surface_variant: Color::from_rgb(0.6, 0.6, 0.6),

    // Primary colors
    primary: Color::from_rgb(0.4, 0.7, 1.0),
    on_primary: Color::from_rgb(0.0, 0.0, 0.0),
    primary_container: Color::from_rgb(0.1, 0.3, 0.5),
    on_primary_container: Color::from_rgb(0.8, 0.9, 1.0),

    // Secondary colors
    secondary: Color::from_rgb(0.6, 0.6, 0.6),
    on_secondary: Color::from_rgb(0.0, 0.0, 0.0),
    secondary_container: Color::from_rgb(0.3, 0.3, 0.3),
    on_secondary_container: Color::from_rgb(0.8, 0.8, 0.8),

    // Error colors
    error: Color::from_rgb(1.0, 0.4, 0.4),
    on_error: Color::from_rgb(0.0, 0.0, 0.0),
    error_container: Color::from_rgb(0.4, 0.1, 0.1),
    on_error_container: Color::from_rgb(1.0, 0.8, 0.8),

    // Success colors
    success: Color::from_rgb(0.4, 0.8, 0.5),
    on_success: Color::from_rgb(0.0, 0.0, 0.0),
    success_container: Color::from_rgb(0.1, 0.3, 0.1),
    on_success_container: Color::from_rgb(0.8, 1.0, 0.8),

    // Warning colors
    warning: Color::from_rgb(1.0, 0.7, 0.2),
    on_warning: Color::from_rgb(0.0, 0.0, 0.0),
    warning_container: Color::from_rgb(0.4, 0.3, 0.1),
    on_warning_container: Color::from_rgb(1.0, 0.9, 0.7),

    // Border colors
    outline: Color::from_rgb(0.4, 0.4, 0.4),
    outline_variant: Color::from_rgb(0.3, 0.3, 0.3),

    // Database-specific colors
    connection_active: Color::from_rgb(0.4, 0.9, 0.4),
    connection_inactive: Color::from_rgb(0.5, 0.5, 0.5),
    query_success: Color::from_rgb(0.4, 0.8, 0.5),
    query_error: Color::from_rgb(1.0, 0.4, 0.4),
    syntax_keyword: Color::from_rgb(0.4, 0.6, 1.0),
    syntax_string: Color::from_rgb(0.6, 0.9, 0.6),
    syntax_comment: Color::from_rgb(0.6, 0.6, 0.6),
    syntax_number: Color::from_rgb(1.0, 0.7, 0.4),
};

/// High contrast theme for accessibility
pub const HIGH_CONTRAST_THEME: ThemeColors = ThemeColors {
    // Background colors
    background: Color::from_rgb(0.0, 0.0, 0.0),
    surface: Color::from_rgb(0.1, 0.1, 0.1),
    surface_variant: Color::from_rgb(0.2, 0.2, 0.2),

    // Text colors
    on_background: Color::from_rgb(1.0, 1.0, 1.0),
    on_surface: Color::from_rgb(1.0, 1.0, 1.0),
    on_surface_variant: Color::from_rgb(0.9, 0.9, 0.9),

    // Primary colors
    primary: Color::from_rgb(0.0, 0.8, 1.0),
    on_primary: Color::from_rgb(0.0, 0.0, 0.0),
    primary_container: Color::from_rgb(0.0, 0.4, 0.5),
    on_primary_container: Color::from_rgb(1.0, 1.0, 1.0),

    // Secondary colors
    secondary: Color::from_rgb(0.8, 0.8, 0.8),
    on_secondary: Color::from_rgb(0.0, 0.0, 0.0),
    secondary_container: Color::from_rgb(0.4, 0.4, 0.4),
    on_secondary_container: Color::from_rgb(1.0, 1.0, 1.0),

    // Error colors
    error: Color::from_rgb(1.0, 0.0, 0.0),
    on_error: Color::from_rgb(1.0, 1.0, 1.0),
    error_container: Color::from_rgb(0.5, 0.0, 0.0),
    on_error_container: Color::from_rgb(1.0, 1.0, 1.0),

    // Success colors
    success: Color::from_rgb(0.0, 1.0, 0.0),
    on_success: Color::from_rgb(0.0, 0.0, 0.0),
    success_container: Color::from_rgb(0.0, 0.5, 0.0),
    on_success_container: Color::from_rgb(1.0, 1.0, 1.0),

    // Warning colors
    warning: Color::from_rgb(1.0, 1.0, 0.0),
    on_warning: Color::from_rgb(0.0, 0.0, 0.0),
    warning_container: Color::from_rgb(0.5, 0.5, 0.0),
    on_warning_container: Color::from_rgb(1.0, 1.0, 1.0),

    // Border colors
    outline: Color::from_rgb(0.8, 0.8, 0.8),
    outline_variant: Color::from_rgb(0.6, 0.6, 0.6),

    // Database-specific colors
    connection_active: Color::from_rgb(0.0, 1.0, 0.0),
    connection_inactive: Color::from_rgb(0.6, 0.6, 0.6),
    query_success: Color::from_rgb(0.0, 1.0, 0.0),
    query_error: Color::from_rgb(1.0, 0.0, 0.0),
    syntax_keyword: Color::from_rgb(0.0, 0.8, 1.0),
    syntax_string: Color::from_rgb(0.0, 1.0, 0.0),
    syntax_comment: Color::from_rgb(0.8, 0.8, 0.8),
    syntax_number: Color::from_rgb(1.0, 0.8, 0.0),
};
