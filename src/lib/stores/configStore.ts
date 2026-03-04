import { writable } from "svelte/store";
import type { Config } from "../types";

const defaultConfig: Config = {
  shortcut: "CmdOrCtrl+Shift+Space",
  shortcut_mode: "toggle",
  autostart: false,
  theme: "ultra-minimal",
};

/** Reactive store for app config — ONLY IPC access point for config data (ARCH-6) */
export const config = writable<Config>(defaultConfig);
