//! Tab buttons to be used in the various pages just under the header

use iced::widget::{button, horizontal_space, Button, Row, Text};
use iced::{alignment, Alignment, Font, Length, Renderer};

use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::{ButtonStyleTuple, ButtonType};
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_SUBTITLE, ICONS};
use crate::gui::styles::text::{TextStyleTuple, TextType};
use crate::gui::types::message::Message;
use crate::{Language, RunningPage, StyleType};

pub fn get_settings_tabs(
    labels: [SettingsPage; 3],
    icons: &[&str],
    actions: &[Message],
    active: SettingsPage,
    font: Font,
    font_headers: Font,
    language: Language,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut tabs = Row::new()
        .width(Length::Fill)
        .align_items(Alignment::Start)
        .spacing(2)
        .padding([0, 3]);

    for (i, label) in labels.iter().enumerate() {
        let active = label.eq(&active);
        tabs = tabs.push(new_tab(
            (*label).get_tab_label(language).to_string(),
            (*icons.get(i).unwrap()).to_string(),
            actions.get(i).unwrap().clone(),
            active,
            font,
            font_headers,
            None,
        ));
    }
    tabs
}

pub fn get_pages_tabs(
    labels: [RunningPage; 3],
    icons: &[&str],
    actions: &[Message],
    active: RunningPage,
    font: Font,
    font_headers: Font,
    language: Language,
    unread_notifications: usize,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut tabs = Row::new()
        .width(Length::Fill)
        .align_items(Alignment::Start)
        .spacing(2)
        .padding([0, 3]);

    for (i, label) in labels.iter().enumerate() {
        let active = label.eq(&active);
        let unread = if label.eq(&RunningPage::Notifications) {
            Some(unread_notifications)
        } else {
            None
        };
        tabs = tabs.push(new_tab(
            (*label).get_tab_label(language).to_string(),
            (*icons.get(i).unwrap()).to_string(),
            actions.get(i).unwrap().clone(),
            active,
            font,
            font_headers,
            unread,
        ));
    }
    tabs
}

fn new_tab(
    label: String,
    icon: String,
    action: Message,
    active: bool,
    font: Font,
    font_headers: Font,
    unread: Option<usize>,
) -> Button<'static, Message, Renderer<StyleType>> {
    let mut content = Row::new()
        .align_items(Alignment::Center)
        .push(horizontal_space(Length::FillPortion(1)))
        .push(
            Text::new(icon)
                .font(ICONS)
                .size(15)
                .style(if active {
                    TextType::Title
                } else {
                    TextType::Standard
                })
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .push(
            Text::new(label)
                .font(font)
                .size(FONT_SIZE_SUBTITLE)
                .style(if active {
                    TextType::Title
                } else {
                    TextType::Standard
                })
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        );

    if let Some(num) = unread {
        if num > 0 {
            let notifications_badge = button(
                Text::new(num.to_string())
                    .font(font_headers)
                    .size(14)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .padding([2, 4, 0, 4])
            .height(Length::Fixed(20.0))
            .style(ButtonType::Badge);
            content = content
                .push(horizontal_space(Length::Fixed(7.0)))
                .push(notifications_badge);
        }
    }

    content = content.push(horizontal_space(Length::FillPortion(1)));

    button(content)
        .height(Length::Fixed(if active { 35.0 } else { 30.0 }))
        .padding(0)
        .width(Length::FillPortion(1))
        .style(if active {
            ButtonType::TabActive
        } else {
            ButtonType::TabInactive
        })
        .on_press(action)
}
