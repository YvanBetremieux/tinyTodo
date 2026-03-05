import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Config } from "../types";

const defaultConfig: Config = {
  shortcut: "CmdOrCtrl+Shift+Space",
  shortcut_mode: "toggle",
  autostart: false,
  theme: "ultra-minimal",
};

/** Reactive store for app config — ONLY IPC access point for config data (ARCH-6) */
export const config = writable<Config>(defaultConfig);

/** Load config from backend */
export async function loadConfig(): Promise<void> {
  const result = await invoke<Config>("get_config");
  config.set(result);
}

/** Save config to backend */
export async function saveConfig(newConfig: Config): Promise<void> {
  await invoke("save_config_command", { config: newConfig });
  config.set(newConfig);
}
