use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent};
use global_hotkey::hotkey::{HotKey, Modifiers, Code};

pub struct ShortcutHandler {
    rx: Receiver<ShortcutEvent>,
    _manager: GlobalHotKeyManager, // Keep manager alive
    last_trigger_time: Arc<Mutex<Option<Instant>>>,
}

#[derive(Debug, Clone)]
pub enum ShortcutEvent {
    ImproveText,
}

impl ShortcutHandler {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let last_trigger_time = Arc::new(Mutex::new(None));
        
        // Create hotkey manager
        let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
        
        // Register Cmd+Shift+E (Super = Cmd on macOS)
        let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyE);
        
        let tx_clone = tx.clone();
        let last_trigger_time_clone = last_trigger_time.clone();
        manager.register(hotkey).expect("Failed to register hotkey");
        
        // Start listening for hotkey events
        thread::spawn(move || {
            let event_channel = GlobalHotKeyEvent::receiver();
            println!("📝 Listening for Cmd+Shift+E to improve selected text (global-hotkey)");
            println!("If nothing happens, ensure you have Accessibility permissions in System Settings > Privacy & Security > Accessibility.");
            
            for _event in event_channel {
                // Debounce: prevent multiple events within 1 second
                let now = Instant::now();
                let mut last_time = last_trigger_time_clone.lock().unwrap();
                
                if let Some(last) = *last_time {
                    let duration = now.duration_since(last);
                    println!("⏱️  Time since last trigger: {:?}", duration);
                    if duration < Duration::from_millis(1000) {
                        println!("⚠️  Ignoring rapid successive hotkey press (debounced)");
                        continue;
                    }
                }
                
                *last_time = Some(now);
                println!("⌨️  Cmd+Shift+E detected!");
                let _ = tx_clone.send(ShortcutEvent::ImproveText);
            }
        });
        
        ShortcutHandler { 
            rx, 
            _manager: manager,
            last_trigger_time,
        }
    }

    pub fn try_receive(&self) -> Option<ShortcutEvent> {
        self.rx.try_recv().ok()
    }
} 