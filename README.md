![Refixy Banner](resources/refixy.png)

# Refixy – AI Writing Assistant for macOS

A lightweight menu bar app that silently improves your writing. Select text, press `Cmd+Shift+E`, and get instant AI-powered corrections.

## ✨ Features

- **Silent Operation**: No popups or distractions
- **System-wide**: Works in any macOS app
- **Instant**: Get improved text in seconds
- **Simple**: Just highlight and press shortcut

## 🚀 Quick Start

1. **Install Rust** from [rustup.rs](https://rustup.rs/)
2. **Get OpenAI API key** from [OpenAI Platform](https://platform.openai.com/)
3. **Clone and run**:
   ```bash
   git clone <repository>
   cd refixy
   export OPENAI_API_KEY="your-api-key-here"
   cargo run
   ```

## 🎯 How to Use

1. Select text in any app
2. Press `Cmd+Shift+E`
3. Get improved text instantly

## 📦 Build

```bash
cargo build --release
```

## 🛠 Tech Stack

- **Rust** + **iced** GUI
- **OpenAI API** for text improvement
- **macOS** native integration

---

*Refixy - Write better, faster.*