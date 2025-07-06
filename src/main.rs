mod executor;
mod shortcuts;
mod clipboard;

use iced::{Application, Command, Element, Settings, Theme, Subscription};
use shortcuts::{ShortcutHandler, ShortcutEvent};
use clipboard::ClipboardManager;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    Enable,
    Disable,
    Quit,
    TextImproved(Result<String, String>),
    ShortcutTriggered,
    Tick,
}

struct RefixApp {
    enabled: bool,
    openai_client: Option<executor::OpenAIClient>,
    shortcut_handler: ShortcutHandler,
    clipboard_manager: ClipboardManager,
}

impl Application for RefixApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // Try to initialize OpenAI client
        let openai_client = executor::OpenAIClient::new().ok();
        
        // Initialize shortcut handler and clipboard manager
        let shortcut_handler = ShortcutHandler::new();
        let clipboard_manager = ClipboardManager::new().expect("Failed to initialize clipboard");
        
        // Start listening for shortcuts
        shortcut_handler.start_listening();
        
        println!("🚀 Refix app started successfully!");
        if openai_client.is_some() {
            println!("✅ OpenAI API connected");
        } else {
            println!("❌ OpenAI API not configured - set OPENAI_API_KEY environment variable");
        }
        
        (
            RefixApp {
                enabled: false,
                openai_client,
                shortcut_handler,
                clipboard_manager,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Refix")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Enable => {
                self.enabled = true;
                println!("🟢 Refix enabled - listening for Cmd+Shift+E...");
                Command::none()
            }
            Message::Disable => {
                self.enabled = false;
                println!("🔴 Refix disabled");
                Command::none()
            }
            Message::Quit => {
                println!("👋 Refix shutting down...");
                std::process::exit(0);
            }
            Message::Tick => {
                // Check for shortcut events
                if let Some(ShortcutEvent::ImproveText) = self.shortcut_handler.try_receive() {
                    return Command::perform(async {}, |_| Message::ShortcutTriggered);
                }
                Command::none()
            }
            Message::ShortcutTriggered => {
                if !self.enabled {
                    println!("⚠️  Refix is disabled - enable it first");
                    return Command::none();
                }
                
                println!("⌨️  Processing Cmd+Shift+E - getting selected text...");
                
                // Get text from clipboard
                let text = match self.clipboard_manager.get_text() {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("❌ Failed to get text from clipboard: {}", e);
                        return Command::none();
                    }
                };
                
                if text.trim().is_empty() {
                    println!("📭 No text selected - copy some text first");
                    return Command::none();
                }
                
                println!("📋 Selected text: \"{}\"", text.trim());
                println!("🤖 Improving text with AI...");
                
                // Improve text using OpenAI
                if let Some(client) = &self.openai_client {
                    let client = client.clone();
                    let text = text.clone();
                    
                    Command::perform(
                        async move {
                            client.improve_text(&text).await
                        },
                        move |result| {
                            Message::TextImproved(result.map_err(|e| e.to_string()))
                        }
                    )
                } else {
                    eprintln!("❌ OpenAI client not available");
                    Command::none()
                }
            }
            Message::TextImproved(result) => {
                match result {
                    Ok(improved_text) => {
                        println!("✨ Text improved successfully!");
                        println!("📝 Improved: \"{}\"", improved_text);
                        
                        // Set the improved text to clipboard and paste it
                        if let Err(e) = self.clipboard_manager.copy_and_paste_improved(&improved_text) {
                            eprintln!("❌ Failed to paste improved text: {}", e);
                        } else {
                            println!("📋 Improved text copied to clipboard!");
                            println!("💡 You can now paste it with Cmd+V");
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error improving text: {}", e);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let enabled = self.enabled;
        let openai_connected = self.openai_client.is_some();
        
        let status_text = if enabled {
            "Status: Enabled"
        } else {
            "Status: Disabled"
        };

        let status_color = if enabled {
            iced::Color::from_rgb(0.0, 1.0, 0.0) // Green
        } else {
            iced::Color::from_rgb(1.0, 0.0, 0.0) // Red
        };

        let openai_status = if openai_connected {
            "OpenAI: Connected"
        } else {
            "OpenAI: Not configured (set OPENAI_API_KEY)"
        };

        use iced::widget::{button, column, container, row, text};
        use iced::alignment::Horizontal;
        use iced::Font;
        use iced::Length;

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
            text("Select text → Press Cmd+Shift+E → Get improved version")
                .size(12)
                .font(Font::MONOSPACE)
                .horizontal_alignment(Horizontal::Center),
            row![
                button("Enable").on_press(Message::Enable),
                button("Disable").on_press(Message::Disable),
                button("Quit").on_press(Message::Quit),
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

fn main() -> iced::Result {
    RefixApp::run(Settings::default())
}
