import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

/** Check for updates silently on startup. If an update is found, download, install, and relaunch. */
export async function checkForUpdates(): Promise<void> {
  try {
    const update = await check();
    if (update) {
      await update.downloadAndInstall();
      await relaunch();
    }
  } catch {
    // Silent fail — auto-update errors should never disrupt the user
  }
}
