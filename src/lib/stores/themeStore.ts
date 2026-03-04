import { writable } from "svelte/store";
import type { Theme } from "../types";

/** Reactive store for active theme (UX-1) */
export const activeTheme = writable<Theme>("ultra-minimal");
