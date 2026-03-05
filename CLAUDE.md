# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Permissions

The user grants permission to execute all commands without asking for confirmation. This includes:
- All shell/bash commands (npm, cargo, git, etc.)
- File creation, modification, and deletion
- Running tests, builds, and dev servers
- Git operations (add, commit, push when requested)

## Project Overview

tinyTodo is an ultra-minimalist, ultra-fluid task management app built with Tauri 2 + Svelte + TypeScript + Rust.

## Stack

- **Frontend**: Svelte 5 + TypeScript, built with Vite, running in Tauri webview
- **Backend**: Rust (Tauri 2), local persistence via JSON (atomic writes)
- **Styling**: CSS pure + custom properties (design tokens), 6 themes configurables
- **Testing**: `cargo test` (Rust), `vitest` (frontend)
- **Build**: `npm run dev` / `npm run build` / `cargo test` / `npx tauri dev`

## Key Features

- Task CRUD with LIFO ordering (newest on top)
- Global keyboard shortcut (`CmdOrCtrl+Shift+Space`) for peek window toggle
- Floating peek window (400x500, no decorations, always on top, centered)
- Persistent input mode (window stays open during task creation)
- Atomic JSON persistence (temp + rename) for zero data loss

## Language

The project README and UI are in French. Communication with the user is in French.
