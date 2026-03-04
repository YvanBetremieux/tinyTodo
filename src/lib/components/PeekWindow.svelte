<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { tasks } from "../stores/taskStore";
  import TaskRow from "./TaskRow.svelte";
  import TaskInput from "./TaskInput.svelte";

  let inputMode = $state(false);

  function activateInputMode() {
    inputMode = true;
    invoke("set_persistent", { persistent: true });
  }

  function deactivateInputMode() {
    inputMode = false;
    invoke("set_persistent", { persistent: false });
  }

  function handleKeydown(e: KeyboardEvent) {
    // Space activates input mode (only when no input is focused)
    if (e.key === " " && !inputMode && document.activeElement?.tagName !== "INPUT") {
      e.preventDefault();
      activateInputMode();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="peek-window">
  <div class="peek-header">
    <button class="add-button" onclick={activateInputMode} title="Nouvelle tâche">+</button>
  </div>

  {#if inputMode}
    <TaskInput onclose={deactivateInputMode} />
  {/if}

  {#if $tasks.length === 0 && !inputMode}
    <p class="empty-state">Rien à faire — profite.</p>
  {:else}
    <ul class="task-list">
      {#each $tasks as task (task.id)}
        <TaskRow {task} />
      {/each}
    </ul>
  {/if}
</div>

<style>
  .peek-window {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .peek-header {
    display: flex;
    justify-content: flex-end;
    padding: var(--space-xs) var(--space-md);
  }

  .add-button {
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    font-size: 20px;
    cursor: pointer;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--border-radius-sm);
    transition: color var(--transition-fast), background-color var(--transition-fast);
    line-height: 1;
  }

  .add-button:hover {
    color: var(--color-accent);
    background-color: var(--color-surface-hover);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    color: var(--color-text-muted);
    font-size: var(--font-size-lg);
  }

  .task-list {
    list-style: none;
    overflow-y: auto;
    flex: 1;
  }
</style>
