//! Text style

use iced::widget::text::Appearance;
use iced::widget::{Column, Text};
use iced::{Color, Font, Renderer};

use crate::gui::styles::style_constants::get_font;
use crate::gui::types::message::Message;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum TextType {
    #[default]
    Standard,
    Incoming,
    Outgoing,
    Title,
    Subtitle,
    Danger,
    Sponsor,
}

/// Returns a formatted caption followed by subtitle, new line, tab, and desc
impl TextType {
    pub fn highlighted_subtitle_with_desc(
        subtitle: &str,
        desc: &str,
        font: Font,
    ) -> Column<'static, Message, Renderer<StyleType>> {
        Column::new()
            .push(
                Text::new(format!("{subtitle}:"))
                    .style(TextType::Subtitle)
                    .font(font),
            )
            .push(Text::new(format!("   {desc}")).font(font))
    }
}

#[derive(Clone)]
pub struct TextStyleTuple(pub StyleType, pub TextType);

impl iced::widget::text::StyleSheet for StyleType {
    type Style = TextType;

    fn appearance(&self, style: Self::Style) -> Appearance {
        Appearance {
            color: if style == TextType::Standard {
                None
            } else {
                Some(highlight(*self, style))
            },
        }
    }
}

/// Returns the weighted average of two colors; color intensity is fixed to 100%
pub fn highlight(style: StyleType, element: TextType) -> Color {
    let colors = get_colors(style);
    let color = colors.secondary;
    let is_nightly = style.is_nightly();
    match element {
        TextType::Title => {
            let (p1, c) = if is_nightly { (0.6, 1.0) } else { (0.9, 0.7) };
            Color {
                r: c * (1.0 - p1) + color.r * p1,
                g: c * (1.0 - p1) + color.g * p1,
                b: c * (1.0 - p1) + color.b * p1,
                a: 1.0,
            }
        }
        TextType::Subtitle => {
            let (p1, c) = if is_nightly { (0.4, 1.0) } else { (0.6, 0.7) };
            Color {
                r: c * (1.0 - p1) + color.r * p1,
                g: c * (1.0 - p1) + color.g * p1,
                b: c * (1.0 - p1) + color.b * p1,
                a: 1.0,
            }
        }
        TextType::Incoming => colors.secondary,
        TextType::Outgoing => colors.outgoing,
        TextType::Danger => Color::from_rgb(0.8, 0.15, 0.15),
        TextType::Sponsor => Color::from_rgb(1.0, 0.3, 0.5),
        _ => colors.text_body,
    }
}
