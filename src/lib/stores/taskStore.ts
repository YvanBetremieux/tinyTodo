import { writable } from "svelte/store";
import type { Task } from "../types";

/** Reactive store for tasks — ONLY IPC access point for task data (ARCH-6) */
export const tasks = writable<Task[]>([]);
