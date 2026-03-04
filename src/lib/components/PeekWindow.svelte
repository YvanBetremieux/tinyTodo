<script lang="ts">
  import { tasks } from "../stores/taskStore";

  const emptyMessage = "Rien à faire — profite.";
</script>

<div class="peek-window">
  {#if $tasks.length === 0}
    <p class="empty-state">{emptyMessage}</p>
  {:else}
    <ul class="task-list">
      {#each $tasks as task (task.id)}
        <li class="task-row">{task.text}</li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .peek-window {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: var(--space-md);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--color-text-muted);
    font-size: var(--font-size-lg);
  }

  .task-list {
    list-style: none;
    overflow-y: auto;
    flex: 1;
  }

  .task-row {
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid var(--color-border);
    color: var(--color-text-primary);
  }
</style>
