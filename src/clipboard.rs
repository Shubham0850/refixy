use arboard::Clipboard;
use std::error::Error;

pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(ClipboardManager)
    }

    pub fn get_text(&self) -> Result<String, Box<dyn Error>> {
        let mut clipboard = Clipboard::new()?;
        let text = clipboard.get_text()?;
        Ok(text)
    }

    pub fn set_text(&self, text: &str) -> Result<(), Box<dyn Error>> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text.to_string())?;
        Ok(())
    }

    pub fn copy_and_paste_improved(&self, improved_text: &str) -> Result<(), Box<dyn Error>> {
        // Set the improved text to clipboard
        self.set_text(improved_text)?;
        
        println!("📋 Improved text copied to clipboard!");
        println!("💡 You can now paste it with Cmd+V");
        
        Ok(())
    }
} 