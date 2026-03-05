<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import PeekWindow from "./lib/components/PeekWindow.svelte";
  import SettingsView from "./lib/components/SettingsView.svelte";
  import { loadTasks } from "./lib/stores/taskStore";
  import { config, loadConfig } from "./lib/stores/configStore";
  import { applyTheme } from "./lib/stores/themeStore";
  import { checkForUpdates } from "./lib/stores/updaterStore";

  let visible = $state(false);
  let settingsMode = $state(false);

  onMount(() => {
    // Load config on startup, then apply saved theme
    loadConfig().then(() => {
      const unsub = config.subscribe((c) => applyTheme(c.theme));
      unsub();
    });

    // Check for updates silently on startup
    checkForUpdates();

    const unlistenShow = listen("show-peek", () => {
      visible = true;
      settingsMode = false;
      loadTasks();
    });

    const unlistenHide = listen("hide-peek", () => {
      visible = false;
      settingsMode = false;
    });

    const unlistenSettings = listen("show-settings", () => {
      visible = true;
      settingsMode = true;
    });

    // Handle Escape key to hide peek
    const handleKeydown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && settingsMode) {
        settingsMode = false;
        loadTasks();
        return;
      }
      if (e.key === "Escape") {
        invoke("hide_peek_command");
      }
    };
    document.addEventListener("keydown", handleKeydown);

    return () => {
      unlistenShow.then((fn) => fn());
      unlistenHide.then((fn) => fn());
      unlistenSettings.then((fn) => fn());
      document.removeEventListener("keydown", handleKeydown);
    };
  });

  function closeSettings() {
    settingsMode = false;
    loadTasks();
  }
</script>

<main class:hidden={!visible}>
  {#if settingsMode}
    <SettingsView onclose={closeSettings} />
  {:else}
    <PeekWindow />
  {/if}
</main>

<style>
  main {
    height: 100%;
  }

  main.hidden {
    visibility: hidden;
  }
</style>
