use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use inputbot::KeybdKey::{EKey, LShiftKey, RShiftKey, LControlKey, RControlKey, LSuperKey, RSuperKey};
use inputbot::handle_input_events;

pub struct ShortcutHandler {
    tx: Sender<ShortcutEvent>,
    rx: Receiver<ShortcutEvent>,
}

#[derive(Debug, Clone)]
pub enum ShortcutEvent {
    ImproveText,
}

impl ShortcutHandler {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        ShortcutHandler { tx, rx }
    }

    pub fn start_listening(&self) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            // Register Cmd+Shift+E (Super = Cmd on macOS)
            EKey.bind(move || {
                if (LSuperKey.is_pressed() || RSuperKey.is_pressed()) &&
                   (LShiftKey.is_pressed() || RShiftKey.is_pressed()) {
                    println!("⌨️  Cmd+Shift+E detected!");
                    let _ = tx.send(ShortcutEvent::ImproveText);
                }
            });
            println!("📝 Listening for Cmd+Shift+E to improve selected text (inputbot)");
            println!("If nothing happens, ensure you have Accessibility permissions in System Settings > Privacy & Security > Accessibility.");
            handle_input_events();
        });
    }

    pub fn try_receive(&self) -> Option<ShortcutEvent> {
        self.rx.try_recv().ok()
    }
} 