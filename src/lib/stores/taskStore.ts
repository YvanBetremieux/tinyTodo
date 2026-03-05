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

/** Create a new task and reload the list */
export async function createTask(text: string): Promise<void> {
  await invoke<Task>("create_task", { text });
  await loadTasks();
}

/** Toggle a task's done status and reload the list */
export async function toggleTask(id: string): Promise<void> {
  await invoke<Task>("toggle_task", { id });
  await loadTasks();
}

/** Update a task's text and reload the list */
export async function updateTask(id: string, text: string): Promise<void> {
  await invoke<Task>("update_task", { id, text });
  await loadTasks();
}
