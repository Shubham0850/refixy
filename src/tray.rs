use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};
use iced::alignment::Horizontal;
use iced::Font;

#[derive(Debug, Clone)]
pub enum TrayMessage {
    Enable,
    Disable,
    Quit,
}

pub struct TrayView {
    pub enabled: bool,
    pub openai_connected: bool,
}

impl TrayView {
    pub fn new(enabled: bool, openai_connected: bool) -> Self {
        TrayView {
            enabled,
            openai_connected,
        }
    }

    pub fn view(&self) -> Element<TrayMessage> {
        let status_text = if self.enabled {
            "Status: Enabled"
        } else {
            "Status: Disabled"
        };

        let status_color = if self.enabled {
            iced::Color::from_rgb(0.0, 1.0, 0.0) // Green
        } else {
            iced::Color::from_rgb(1.0, 0.0, 0.0) // Red
        };

        let openai_status = if self.openai_connected {
            "OpenAI: Connected"
        } else {
            "OpenAI: Not configured"
        };

        let content = column![
            text("Refix - AI Writing Assistant")
                .size(24)
                .font(Font::MONOSPACE)
                .horizontal_alignment(Horizontal::Center),
            text(status_text)
                .size(16)
                .font(Font::MONOSPACE)
                .style(status_color)
                .horizontal_alignment(Horizontal::Center),
            text(openai_status)
                .size(12)
                .font(Font::MONOSPACE)
                .horizontal_alignment(Horizontal::Center),
            text("Press Cmd+Shift+E to improve selected text")
                .size(12)
                .font(Font::MONOSPACE)
                .horizontal_alignment(Horizontal::Center),
            row![
                button("Enable").on_press(TrayMessage::Enable),
                button("Disable").on_press(TrayMessage::Disable),
                button("Quit").on_press(TrayMessage::Quit),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(20)
        .padding(20)
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
} 