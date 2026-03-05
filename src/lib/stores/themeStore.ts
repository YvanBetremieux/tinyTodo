import { writable } from "svelte/store";
import type { Theme } from "../types";

/** Reactive store for active theme (UX-1) */
export const activeTheme = writable<Theme>("ultra-minimal");

/** Available themes with display labels */
export const THEMES: { id: Theme; label: string }[] = [
  { id: "ultra-minimal", label: "Ultra-Minimal" },
  { id: "floating-card", label: "Floating Card" },
  { id: "glass", label: "Glass" },
  { id: "macos-native", label: "macOS Native" },
  { id: "high-contrast", label: "Contraste élevé" },
  { id: "warm", label: "Warm" },
];

/** Apply theme by setting data-theme attribute on :root */
export function applyTheme(theme: Theme): void {
  document.documentElement.setAttribute("data-theme", theme);
  activeTheme.set(theme);
}
