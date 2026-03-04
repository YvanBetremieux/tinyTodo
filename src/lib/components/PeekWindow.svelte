<script lang="ts">
  import { tasks } from "../stores/taskStore";
  import TaskRow from "./TaskRow.svelte";
</script>

<div class="peek-window">
  {#if $tasks.length === 0}
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
</style>
