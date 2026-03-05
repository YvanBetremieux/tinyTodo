<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { tasks } from "../stores/taskStore";
  import { toggleTask, reorderTasks } from "../stores/taskStore";
  import TaskRow from "./TaskRow.svelte";
  import TaskInput from "./TaskInput.svelte";
  import HistoryView from "./HistoryView.svelte";

  let inputMode = $state(false);
  let historyMode = $state(false);
  let selectedIndex = $state(-1);
  let dragFromIndex = $state(-1);
  let dragOverIndex = $state(-1);

  function activateInputMode() {
    inputMode = true;
    selectedIndex = -1;
    invoke("set_persistent", { persistent: true });
  }

  function deactivateInputMode() {
    inputMode = false;
    invoke("set_persistent", { persistent: false });
  }

  function toggleHistoryMode() {
    historyMode = !historyMode;
    selectedIndex = -1;
  }

  function closeHistory() {
    historyMode = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (historyMode) return; // HistoryView handles its own Escape

    const isInputFocused = document.activeElement?.tagName === "INPUT";

    if (e.key === " " && !inputMode && !isInputFocused) {
      e.preventDefault();
      activateInputMode();
      return;
    }

    if (isInputFocused || inputMode) return;

    const taskCount = $tasks.length;
    if (taskCount === 0) return;

    if (e.key === "ArrowDown") {
      e.preventDefault();
      if (selectedIndex < taskCount - 1) {
        selectedIndex++;
      }
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      if (selectedIndex > 0) {
        selectedIndex--;
      }
    } else if (e.key === "Enter" && selectedIndex >= 0 && selectedIndex < taskCount) {
      e.preventDefault();
      const task = $tasks[selectedIndex];
      toggleTask(task.id);
      if (selectedIndex >= taskCount - 1) {
        selectedIndex = Math.max(-1, taskCount - 2);
      }
    }
  }

  function handleDragStart(index: number) {
    dragFromIndex = index;
    selectedIndex = -1;
  }

  function handleDragOver(index: number) {
    dragOverIndex = index;
  }

  async function handleDrop(toIndex: number) {
    if (dragFromIndex >= 0 && dragFromIndex !== toIndex) {
      const currentTasks = [...$tasks];
      const [moved] = currentTasks.splice(dragFromIndex, 1);
      currentTasks.splice(toIndex, 0, moved);
      const newOrder = currentTasks.map(t => t.id);
      await reorderTasks(newOrder);
    }
    dragFromIndex = -1;
    dragOverIndex = -1;
  }

  function handleDragEnd() {
    dragFromIndex = -1;
    dragOverIndex = -1;
  }

  // Reset selection when tasks change
  $effect(() => {
    const taskCount = $tasks.length;
    if (selectedIndex >= taskCount) {
      selectedIndex = Math.max(-1, taskCount - 1);
    }
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="peek-window">
  {#if historyMode}
    <HistoryView onclose={closeHistory} />
  {:else}
    <div class="peek-header">
      <button class="header-button" onclick={toggleHistoryMode} title="Historique">⏱</button>
      <button class="header-button add-button" onclick={activateInputMode} title="Nouvelle tâche">+</button>
    </div>

    {#if inputMode}
      <TaskInput onclose={deactivateInputMode} />
    {/if}

    {#if $tasks.length === 0 && !inputMode}
      <p class="empty-state">Rien à faire — profite.</p>
    {:else}
      <ul class="task-list">
        {#each $tasks as task, i (task.id)}
          <TaskRow
            {task}
            selected={i === selectedIndex}
            dragging={i === dragFromIndex}
            dragOver={i === dragOverIndex && i !== dragFromIndex}
            ondragstart={() => handleDragStart(i)}
            ondragover={() => handleDragOver(i)}
            ondrop={() => handleDrop(i)}
            ondragend={handleDragEnd}
          />
        {/each}
      </ul>
    {/if}
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
    gap: var(--space-xs);
    padding: var(--space-xs) var(--space-md);
  }

  .header-button {
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

  .header-button:hover {
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
