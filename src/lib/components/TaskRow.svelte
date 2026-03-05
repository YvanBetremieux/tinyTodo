<script lang="ts">
  import type { Task } from "../types";
  import { toggleTask } from "../stores/taskStore";

  interface Props {
    task: Task;
  }

  let { task }: Props = $props();
  let fadingOut = $state(false);

  async function handleToggle() {
    fadingOut = true;
    // Wait for fade-out animation (~250ms) before actually toggling
    setTimeout(async () => {
      await toggleTask(task.id);
    }, 250);
  }
</script>

<li class="task-row" class:fade-out={fadingOut}>
  <input
    type="checkbox"
    class="task-checkbox"
    checked={task.done}
    onchange={handleToggle}
  />
  <span class="task-text">{task.text}</span>
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
  }
</style>
