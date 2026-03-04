<script lang="ts">
  import type { Task } from "../types";

  interface Props {
    task: Task;
  }

  let { task }: Props = $props();
</script>

<li class="task-row">
  <input type="checkbox" class="task-checkbox" checked={task.done} disabled />
  <span class="task-text">{task.text}</span>
</li>

<style>
  .task-row {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid var(--color-border);
    transition: background-color var(--transition-fast);
  }

  .task-row:hover {
    background-color: var(--color-surface-hover);
  }

  .task-checkbox {
    appearance: none;
    width: 18px;
    height: 18px;
    min-width: 18px;
    border: 2px solid var(--color-checkbox);
    border-radius: var(--border-radius-sm);
    cursor: default;
    background: transparent;
  }

  .task-text {
    color: var(--color-text-primary);
    font-size: var(--font-size-base);
    line-height: 1.4;
    word-break: break-word;
  }
</style>
