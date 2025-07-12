use arboard::Clipboard;
use get_selected_text::get_selected_text;
use enigo::{Enigo, KeyboardControllable};
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(ClipboardManager)
    }

    pub fn get_text(&self) -> Result<String, Box<dyn Error>> {
        // Get the currently selected text from the active application
        let selected_text = get_selected_text()
            .map_err(|e| format!("Failed to get selected text: {}", e))?;
        
        if selected_text.trim().is_empty() {
            return Err("No text selected".into());
        }
        
        Ok(selected_text)
    }

    pub fn set_text(&self, text: &str) -> Result<(), Box<dyn Error>> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text.to_string())?;
        Ok(())
    }

    pub fn copy_and_paste_improved(&self, improved_text: &str) -> Result<(), Box<dyn Error>> {
        // Set the improved text to clipboard
        self.set_text(improved_text)?;
        
        // Small delay to ensure clipboard is updated
        thread::sleep(Duration::from_millis(100));
        
        // Simulate Cmd+V to paste the improved text
        // This will replace the currently selected text
        self.simulate_paste()?;
        
        println!("✨ Text automatically replaced with improved version!");
        
        Ok(())
    }
    
    fn simulate_paste(&self) -> Result<(), Box<dyn Error>> {
        let mut enigo = Enigo::new();
        
        // Simulate Cmd+V (Super+V on macOS)
        enigo.key_down(enigo::Key::Meta); // Cmd key
        enigo.key_click(enigo::Key::Layout('v')); // V key
        enigo.key_up(enigo::Key::Meta); // Release Cmd key
        
        Ok(())
    }
} 