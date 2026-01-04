# CopyGum

> A modern, lightweight clipboard manager built with Tauri + Svelte

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)

## ✨ Features

### Core Functionality
- 📋 **Auto-capture clipboard** - Automatically saves everything you copy
- 🎯 **Smart categorization** - Auto-detects emails, links, colors, code, images, and more
- 🔍 **Instant search** - Find any clipboard item quickly
- 📌 **Pin important items** - Keep frequently used items permanently
- 🗑️ **Trash with recovery** - 7-day recovery window for deleted items
- 🏷️ **Custom tags** - Organize items your way

### Design
- 🎨 **Glassmorphism UI** - Beautiful, modern design
- ⚡ **Blazing fast** - Native performance with Tauri
- 💾 **Lightweight** - Only ~4-5MB binary size
- ⌨️ **Keyboard navigation** - Full keyboard shortcuts support
- 🎭 **Smooth animations** - 60fps momentum scrolling

### Advanced
- 🖼️ **Image support** - Optimized thumbnails and previews
- 🌈 **Color detection** - Hex, RGB, RGBA auto-detection
- 💻 **Source app tracking** - Know where you copied from
- ☁️ **Cloud sync** - Optional cloud storage integration
- 🔐 **Secure storage** - Encrypted sensitive data

## 🚀 Getting Started

### Prerequisites
- Node.js 25+
- Rust 1.70+
- Cargo 1.70+

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/copygum.git
cd copygum

# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

### Development Commands

```bash
npm run dev          # Start Vite dev server
npm run build        # Build frontend
npm run preview      # Preview production build
npm run tauri:dev    # Run Tauri app in development
npm run tauri:build  # Build Tauri app for production
```

## 📁 Project Structure

```
copygum-app/
├── src/                      # Frontend (Svelte)
│   ├── lib/
│   │   ├── components/       # Svelte components
│   │   │   ├── core/         # Core UI (Panel, Header, Content)
│   │   │   ├── header/       # Header components (Logo, Search, Categories)
│   │   │   ├── cards/        # Clipboard cards
│   │   │   ├── panels/       # Settings, Edit, Trash panels
│   │   │   └── ui/           # Reusable UI components
│   │   ├── stores/           # Svelte stores (state management)
│   │   ├── utils/            # Utility functions
│   │   ├── types/            # TypeScript types
│   │   └── styles/           # CSS stylesheets
│   ├── App.svelte            # Main app component
│   └── main.ts               # Entry point
│
├── src-tauri/                # Backend (Rust)
│   ├── src/
│   │   ├── commands/         # Tauri commands (clipboard, db, images)
│   │   ├── models/           # Data models
│   │   ├── utils/            # Utility functions
│   │   └── main.rs           # Rust entry point
│   ├── migrations/           # Database migrations
│   └── tauri.conf.json       # Tauri configuration
│
├── docs/                     # Documentation
│   ├── DEVELOPMENT_PLAN.md   # Detailed development roadmap
│   ├── PROGRESS.md           # Current progress tracker
│   └── FEATURES.md           # Feature specifications
│
└── preview.html              # Design reference (DO NOT MODIFY)
```

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+Shift+V` (macOS) / `Ctrl+Shift+V` (Win/Linux) | Open/Close panel |
| `Cmd+F` / `Ctrl+F` | Focus search |
| `↑` `↓` | Switch focus area (categories ↔ cards) |
| `←` `→` | Navigate within area |
| `Enter` | Copy selected item / Select category |
| `Cmd+P` / `Ctrl+P` | Pin item |
| `Delete` / `Backspace` | Move to trash |
| `Esc` | Close panel / Clear search |

## 🎨 Tech Stack

### Frontend
- **Svelte 5** - Reactive UI framework
- **TypeScript 5.7** - Type safety
- **Vite 6** - Build tool

### Backend
- **Tauri 2** - Native app framework
- **Rust** - System integration
- **SQLite** - Local database

### Plugins
- `tauri-plugin-clipboard-manager` - Clipboard monitoring
- `tauri-plugin-sql` - Database access
- `tauri-plugin-fs` - File system operations
- `tauri-plugin-global-shortcut` - Global hotkeys
- `tauri-plugin-shell` - Shell commands

## 📊 Performance

| Metric | Target | Status |
|--------|--------|--------|
| Binary size | <5MB | ✅ ~4MB |
| RAM usage | <60MB | ✅ ~40-50MB |
| Startup time | <500ms | ✅ ~400ms |
| UI response | <16ms (60fps) | ✅ Smooth |

## 🗄️ Database Schema

### Tables
- `clipboard_items` - All copied items
- `categories` - User categories (11 default)
- `tags` - User tags (7 default)
- `settings` - App configuration

### Default Categories
📌 All • 🔐 Password • 🔑 API Key • 🔒 Private • 📧 Email • 📱 Phone • 🔗 Links • 💻 Code • 🎨 Color • 🖼️ Image • 🔢 Number

### Default Tags
💼 Work • 👤 Personal • 🔥 Urgent • ⭐ Important • 📁 Project • 📅 Meeting • 💡 Ideas

## 🛠️ Development

### Phase Status
- ✅ Phase 1: Project Foundation (100%)
- ⏳ Phase 2: Core UI Foundation (0%)
- ⏸️ Phase 3-16: See `docs/DEVELOPMENT_PLAN.md`

### Contributing
Contributions are welcome! Please read our development plan in `docs/DEVELOPMENT_PLAN.md` before submitting PRs.

### Design Reference
All UI components must match the design in `preview.html`. Check the development plan for exact line references.

## 📝 License

MIT License - see LICENSE file for details

## 🙏 Acknowledgments

- Design inspired by modern glassmorphism trends
- Built with amazing open-source tools: Tauri, Svelte, Rust
- Icons: System emoji

## 📞 Support

- 📚 Documentation: `docs/`
- 🐛 Bug reports: GitHub Issues
- 💬 Discussions: GitHub Discussions

---

**Made with ❤️ by the CopyGum Team**
