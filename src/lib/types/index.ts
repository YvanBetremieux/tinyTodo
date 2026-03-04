/** Task data structure — mirrors Rust Task struct */
export interface Task {
  id: string;
  text: string;
  done: boolean;
  created_at: string;
  completed_at: string | null;
  sort_order: number;
}

/** App configuration — mirrors Rust Config struct */
export interface Config {
  shortcut: string;
  shortcut_mode: "toggle" | "hold";
  autostart: boolean;
  theme: Theme;
}

/** Available themes (UX-1) */
export type Theme =
  | "ultra-minimal"
  | "floating-card"
  | "glass"
  | "macos-native"
  | "high-contrast"
  | "warm";
