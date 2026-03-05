# tinyTodo

Ultra-minimalist, ultra-fluid task management app. Lives in your system tray, pops up with a global shortcut, and stays out of your way.

## Features

- **Global shortcut** (`Cmd+Shift+Space`) — summon the task list from anywhere
- **Quick capture** — press Space to add a task in seconds
- **Keyboard-first** — navigate with arrows, check with Enter, drag to reorder
- **Inline editing** — double-click any task to edit it
- **History** — browse completed tasks and undo if needed
- **6 themes** — Ultra-Minimal, Floating Card, Glass, macOS Native, High Contrast, Warm
- **System tray** — always running, access settings from the tray icon
- **Auto-start** — launches with your OS (optional)
- **Auto-update** — silently updates via GitHub Releases

## Install

### Download (recommended)

Go to the [Releases page](https://github.com/YvanBetremieux/tinyTodo/releases) and download the latest version for your platform:

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `.dmg` (aarch64) |
| macOS (Intel) | `.dmg` (x86_64) |
| Windows | `.msi` |
| Linux | `.AppImage` or `.deb` |

Open the downloaded file and follow the installer. The app auto-updates after that.

### Build from source

**Prerequisites:** [Node.js](https://nodejs.org/) (LTS), [Rust](https://rustup.rs/), and the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS.

```bash
git clone https://github.com/YvanBetremieux/tinyTodo.git
cd tinyTodo
npm install
npm run tauri dev     # development mode
npm run tauri build   # production build
```

## Usage

1. The app starts in the system tray (no window on launch)
2. Press **Cmd+Shift+Space** (or Ctrl+Shift+Space on Windows/Linux) to show/hide the task list
3. Press **Space** to enter a new task, then **Enter** to save
4. Click the checkbox or select with arrows + **Enter** to complete a task
5. Right-click the tray icon for **Settings** or **Quit**

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `Cmd+Shift+Space` | Show/hide task list |
| `Space` | New task |
| `↑` `↓` | Navigate tasks |
| `Enter` | Complete selected task |
| `Escape` | Close input / go back / hide |
| Double-click | Edit a task |
| Drag | Reorder tasks |

## Tech stack

- **Tauri 2** + **Rust** backend
- **Svelte 5** + **TypeScript** frontend
- **JSON** local persistence (atomic writes)
- **GitHub Actions** CI/CD with multi-platform builds

## License

ISC
