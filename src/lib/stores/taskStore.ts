import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Task } from "../types";

/** Reactive store for tasks — ONLY IPC access point for task data (ARCH-6) */
export const tasks = writable<Task[]>([]);

/** Load active tasks from backend */
export async function loadTasks(): Promise<void> {
  const result = await invoke<Task[]>("get_tasks");
  tasks.set(result);
}
