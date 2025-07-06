# Refix – Your Silent AI-Powered Writing Assistant (macOS)

Refix is a lightweight macOS menu bar app that quietly corrects your writing.  
Select any text → press `Cmd+Shift+E` → your draft gets instantly refactored via AI.

---

## 🚀 Why Refix?

Grammarly is powerful but intrusive.  
Refix works system-wide, without any underlines, popups, or distractions.  
Just highlight text, press a shortcut, and Refix silently improves it.

---

## 🛠 Tech Stack

- **Language**: Rust 🦀  
- **GUI**: [`iced`](https://github.com/iced-rs/iced) `v0.12.1` with `canvas`, `tokio`
- **AI**: OpenAI API
- **System Access**:
  - Clipboard (copy/paste)
  - Global keyboard shortcut listener (planned)
- **Packaging**: macOS `.app` using `cargo-bundle` or `dmg`

---

## 📦 Dependencies

```toml
[dependencies]
iced = { version = "0.12.1", features = ["canvas", "tokio"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"
arboard = "2.0"
tao = "0.24"
```

---

## ✨ Features (Current Implementation)

* [x] Menu bar app with "Enable", "Disable", and "Quit"
* [x] OpenAI API integration for text improvement
* [x] Clipboard reading and writing
* [x] Status display (enabled/disabled, OpenAI connection)
* [x] Silent background operation
* [x] OpenAI API key stored via environment variable
* [ ] Global shortcut: `Cmd+Shift+E` (simulated for testing)
* [ ] Automatic paste after improvement

---

## ⚙️ How it Works

1. User selects text in any macOS app
2. Refix listens for `Cmd+Shift+E` (currently simulated every 5 seconds for testing)
3. Copies text to clipboard
4. Sends it to OpenAI with a prompt like:

   > Improve grammar, tone, and flow without changing the meaning:
5. Receives corrected version
6. Replaces original text via simulated `Cmd+V`

---

## 🧪 Example Use

> User types in Notes:
> `i hope you are doing good i was wondering if we can connect`

> Presses `Cmd+Shift+E`
> ✅ Output: `I hope you're doing well! I was wondering if we could connect.`

---

## 📁 Project Structure

```
refix/
├── src/
│   ├── main.rs          # Main application logic
│   ├── executor.rs      # OpenAI API integration
│   ├── shortcuts.rs     # Global shortcut handling
│   ├── clipboard.rs     # Clipboard operations
│   └── tray.rs          # Menu bar UI (unused)
├── assets/
│   └── icon.png
├── Cargo.toml
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites

1. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)
2. **OpenAI API Key**: Get your API key from [OpenAI Platform](https://platform.openai.com/)

### Setup

1. **Clone and build**:
   ```bash
   git clone <repository>
   cd refix
   cargo build
   ```

2. **Set your OpenAI API key**:
   ```bash
   export OPENAI_API_KEY="your-api-key-here"
   ```

3. **Run the app**:
   ```bash
   cargo run
   ```

### Usage

1. **Start the app**: Run `cargo run` with your API key set
2. **Enable the service**: Click "Enable" in the app window
3. **Test the functionality**: 
   - Copy some text to your clipboard
   - Wait 5 seconds (simulated shortcut trigger)
   - Check your clipboard for improved text
4. **Use in real apps**: The improved text will be in your clipboard, ready to paste

---

## 🧠 Prompt Used

```
You are a helpful writing assistant. Improve the grammar, tone, and flow of the following message without changing its meaning:
"{user_text}"
```

---

## 🧳 Packaging for macOS

Use:

```sh
cargo install cargo-bundle
cargo bundle --release
```

Result: `.app` bundle for macOS distribution.

Optional: Create `.dmg` using tools like [`create-dmg`](https://github.com/sindresorhus/create-dmg)

---

## 🛡 Out of Scope for MVP

* No authentication
* No style/tone options
* No settings UI
* No multi-language support

---

## ✅ Success Criteria

* ⌨️ Shortcut instantly refactors selected text
* 🖥 Works across all apps (Slack, Mail, Notes, etc.)
* 🧠 Feels native to macOS users
* 💨 Response under 2s with good internet

---

## 📌 Current Status

✅ Name: `Refix`
✅ Icon: ✔️
✅ MVP plan: ✅
✅ Basic implementation: ✅
✅ OpenAI integration: ✅
✅ Clipboard operations: ✅
🧱 Global shortcuts: In progress (simulated for testing)
🧱 Automatic paste: TODO

---

## 🔧 Development Notes

### Current Limitations

1. **Global Shortcuts**: The `tao` crate version doesn't include global shortcut support. For now, shortcuts are simulated every 5 seconds for testing.

2. **Automatic Paste**: The clipboard is updated with improved text, but automatic paste simulation requires additional system-level keyboard simulation.

3. **Menu Bar Integration**: Currently runs as a regular window app. Menu bar integration requires additional setup.

### Next Steps

1. **Implement proper global shortcuts** using a different approach or updated dependencies
2. **Add automatic paste simulation** using system-level keyboard events
3. **Convert to true menu bar app** with proper macOS integration
4. **Add error handling and user feedback**
5. **Implement proper configuration management**

---

## 🐛 Known Issues

- Global shortcuts are simulated (not real system-wide shortcuts)
- No automatic paste after improvement
- Runs as window app instead of menu bar app
- Limited error handling and user feedback