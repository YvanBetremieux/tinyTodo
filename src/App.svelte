<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import PeekWindow from "./lib/components/PeekWindow.svelte";
  import { loadTasks } from "./lib/stores/taskStore";

  let visible = $state(false);

  onMount(() => {
    const unlistenShow = listen("show-peek", () => {
      visible = true;
      loadTasks();
    });

    const unlistenHide = listen("hide-peek", () => {
      visible = false;
    });

    // Handle Escape key to hide peek
    const handleKeydown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        invoke("hide_peek_command");
      }
    };
    document.addEventListener("keydown", handleKeydown);

    return () => {
      unlistenShow.then((fn) => fn());
      unlistenHide.then((fn) => fn());
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<main class:hidden={!visible}>
  <PeekWindow />
</main>

<style>
  main {
    height: 100%;
  }

  main.hidden {
    visibility: hidden;
  }
</style>
