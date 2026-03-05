<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Task } from "../types";
  import { toggleTask, updateTask } from "../stores/taskStore";

  interface Props {
    task: Task;
    selected?: boolean;
  }

  let { task, selected = false }: Props = $props();
  let fadingOut = $state(false);
  let editing = $state(false);
  let editText = $state("");
  let editInputEl: HTMLInputElement | undefined = $state();
  let rowEl: HTMLLIElement | undefined = $state();

  async function handleToggle() {
    fadingOut = true;
    setTimeout(async () => {
      await toggleTask(task.id);
    }, 250);
  }

  function startEditing() {
    editing = true;
    editText = task.text;
    invoke("set_persistent", { persistent: true });
  }

  async function saveEdit() {
    const trimmed = editText.trim();
    if (trimmed && trimmed !== task.text) {
      await updateTask(task.id, trimmed);
    }
    stopEditing();
  }

  function cancelEdit() {
    stopEditing();
  }

  function stopEditing() {
    editing = false;
    editText = "";
    invoke("set_persistent", { persistent: false });
  }

  function handleEditKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      saveEdit();
    } else if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      cancelEdit();
    }
  }

  $effect(() => {
    if (editing && editInputEl) {
      editInputEl.focus();
      editInputEl.select();
    }
  });

  // Scroll into view when selected
  $effect(() => {
    if (selected && rowEl) {
      rowEl.scrollIntoView({ block: "nearest" });
    }
  });
</script>

<li
  bind:this={rowEl}
  class="task-row"
  class:fade-out={fadingOut}
  class:selected
>
  <input
    type="checkbox"
    class="task-checkbox"
    checked={task.done}
    onchange={handleToggle}
  />
  {#if editing}
    <input
      bind:this={editInputEl}
      bind:value={editText}
      onkeydown={handleEditKeydown}
      onblur={saveEdit}
      type="text"
      class="edit-field"
    />
  {:else}
    <span class="task-text" ondblclick={startEditing}>{task.text}</span>
  {/if}
</li>

<style>
  .task-row {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid var(--color-border);
    transition: background-color var(--transition-fast), opacity 250ms ease-out;
  }

  .task-row:hover {
    background-color: var(--color-surface-hover);
  }

  .task-row.selected {
    background-color: var(--color-surface-selected, rgba(255, 255, 255, 0.08));
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
  }

  .task-row.fade-out {
    opacity: 0;
  }

  .task-checkbox {
    appearance: none;
    width: 18px;
    height: 18px;
    min-width: 18px;
    border: 2px solid var(--color-checkbox);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    background: transparent;
    transition: background-color var(--transition-fast), border-color var(--transition-fast);
  }

  .task-checkbox:hover {
    border-color: var(--color-accent);
  }

  .task-checkbox:checked {
    background-color: var(--color-accent);
    border-color: var(--color-accent);
  }

  .task-text {
    color: var(--color-text-primary);
    font-size: var(--font-size-base);
    line-height: 1.4;
    word-break: break-word;
    cursor: text;
    flex: 1;
  }

  .edit-field {
    flex: 1;
    background: transparent;
    border: none;
    border-bottom: 2px solid var(--color-accent);
    outline: none;
    color: var(--color-text-primary);
    font-family: var(--font-family);
    font-size: var(--font-size-base);
    padding: 0;
    line-height: 1.4;
  }
</style>
