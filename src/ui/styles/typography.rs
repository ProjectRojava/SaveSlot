//! Typography definitions for consistent text styling

use iced::Font;
use iced::font::{Family, Weight};

/// Font families used in the application
pub mod fonts {
    use super::*;

    pub const DEFAULT: Font = Font {
        family: Family::SansSerif,
        weight: Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    };

    pub const MONOSPACE: Font = Font {
        family: Family::Monospace,
        weight: Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    };

    pub const BOLD: Font = Font {
        family: Family::SansSerif,
        weight: Weight::Bold,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    };

    pub const LIGHT: Font = Font {
        family: Family::SansSerif,
        weight: Weight::Light,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    };
}

/// Font sizes used throughout the application
pub mod sizes {
    pub const TINY: u16 = 10;
    pub const SMALL: u16 = 12;
    pub const BODY: u16 = 14;
    pub const SUBTITLE: u16 = 16;
    pub const TITLE: u16 = 18;
    pub const HEADING: u16 = 20;
    pub const LARGE_HEADING: u16 = 24;
    pub const DISPLAY: u16 = 32;
}

/// Line height values for different text types
pub mod line_heights {
    use iced::widget::text::LineHeight;

    pub const TIGHT: LineHeight = LineHeight::Relative(1.2);
    pub const NORMAL: LineHeight = LineHeight::Relative(1.4);
    pub const RELAXED: LineHeight = LineHeight::Relative(1.6);
    pub const LOOSE: LineHeight = LineHeight::Relative(1.8);
}

/// Common text styles for different UI elements
pub mod text_styles {
    use super::*;
    use iced::widget::Text;

    pub fn heading<'a>(content: &'a str) -> Text<'a> {
        Text::new(content)
            .font(fonts::BOLD)
            .size(sizes::HEADING)
            .line_height(line_heights::TIGHT)
    }

    pub fn title<'a>(content: &'a str) -> Text<'a> {
        Text::new(content)
            .font(fonts::BOLD)
            .size(sizes::TITLE)
            .line_height(line_heights::NORMAL)
    }

    pub fn subtitle<'a>(content: &'a str) -> Text<'a> {
        Text::new(content)
            .font(fonts::DEFAULT)
            .size(sizes::SUBTITLE)
            .line_height(line_heights::NORMAL)
    }

    pub fn body<'a>(content: &'a str) -> Text<'a> {
        Text::new(content)
            .font(fonts::DEFAULT)
            .size(sizes::BODY)
            .line_height(line_heights::NORMAL)
    }

    pub fn small<'a>(content: &'a str) -> Text<'a> {
        Text::new(content)
            .font(fonts::DEFAULT)
            .size(sizes::SMALL)
            .line_height(line_heights::NORMAL)
    }

    pub fn code<'a>(content: &'a str) -> Text<'a> {
        Text::new(content)
            .font(fonts::MONOSPACE)
            .size(sizes::BODY)
            .line_height(line_heights::RELAXED)
    }

    pub fn caption<'a>(content: &'a str) -> Text<'a> {
        Text::new(content)
            .font(fonts::LIGHT)
            .size(sizes::SMALL)
            .line_height(line_heights::NORMAL)
    }
}
