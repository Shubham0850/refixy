# Refixy – AI Writing Assistant

Select text, press `Cmd+Shift+E`, get instant AI-powered improvements.

## Setup

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Get OpenAI API key from [OpenAI Platform](https://platform.openai.com/)
3. Run:
   ```bash
   export OPENAI_API_KEY="your-api-key"
   cargo run
   ```

## Usage

1. Select text in any app
2. Press `Cmd+Shift+E`
3. Text is automatically improved and pasted

## Build

```bash
cargo build --release
```

## Tech

### System Architecture

Refixy is a native macOS application built with Rust and the Iced GUI framework. The system follows a modular architecture with clear separation of concerns:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Global Hotkey │    │   Text Capture  │    │   AI Processing │
│   Detection     │───▶│   & Clipboard   │───▶│   (OpenAI API)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Event Handler │    │   Text Sanitize │    │   Auto Paste    │
│   (Debounced)   │    │   & Validation  │    │   (Keyboard)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Data Flow

1. **Hotkey Detection**: `global-hotkey` library listens for `Cmd+Shift+E` system-wide
2. **Text Selection**: `get-selected-text` captures currently selected text from active application
3. **AI Processing**: Selected text is sent to OpenAI GPT-4o-mini via HTTPS
4. **Response Sanitization**: AI response is cleaned (removes quotes, prefixes, etc.)
5. **Auto Paste**: `enigo` library simulates `Cmd+V` to replace selected text

### Security

- **No Data Storage**: Text is processed in-memory only, never stored locally
- **HTTPS Only**: All API communication uses encrypted HTTPS connections
- **API Key Security**: OpenAI API key stored in environment variables only
- **Local Processing**: All text processing happens locally before API calls
- **No Logging**: No sensitive text is logged to console or files

### Technical Stack

**Core Framework:**
- **Rust** (1.71+) - Memory-safe systems programming
- **Iced** (0.12.1) - Cross-platform GUI framework
- **Tokio** - Async runtime for non-blocking operations

**System Integration:**
- **global-hotkey** (0.7) - Cross-platform global hotkey detection
- **get-selected-text** (0.1) - Native text selection capture
- **enigo** (0.1) - Cross-platform keyboard/mouse simulation
- **arboard** (2.0) - Clipboard management

**AI Integration:**
- **reqwest** (0.11) - HTTP client for API calls
- **serde_json** (1.0) - JSON serialization/deserialization
- **OpenAI GPT-4o-mini** - Text improvement model

### Performance

- **Debounced Events**: 1-second debounce prevents duplicate processing
- **Async Processing**: Non-blocking UI during API calls
- **Memory Efficient**: Minimal memory footprint (~5MB)
- **Fast Response**: Typically 1-3 seconds for text improvement

### Platform Support

Currently optimized for **macOS** with:
- Native accessibility permissions
- System-wide hotkey detection
- Cross-application text selection
- Automatic clipboard integration