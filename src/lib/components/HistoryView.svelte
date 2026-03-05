<script lang="ts">
  import { history, loadHistory, toggleTask } from "../stores/taskStore";

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();

  // Load history on mount
  loadHistory();

  function formatDate(isoDate: string | null): string {
    if (!isoDate) return "";
    const date = new Date(isoDate);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMin = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMin < 1) return "à l'instant";
    if (diffMin < 60) return `il y a ${diffMin}min`;
    if (diffHours < 24) return `il y a ${diffHours}h`;
    if (diffDays < 7) return `il y a ${diffDays}j`;

    return date.toLocaleDateString("fr-FR", { day: "numeric", month: "short" });
  }

  async function handleUncomplete(id: string) {
    await toggleTask(id);
    await loadHistory();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="history-view">
  <div class="history-header">
    <button class="back-button" onclick={onclose} title="Retour">←</button>
    <span class="history-title">Historique</span>
  </div>

  {#if $history.length === 0}
    <p class="empty-state">Aucune tâche accomplie.</p>
  {:else}
    <ul class="history-list">
      {#each $history as task (task.id)}
        <li class="history-row">
          <input
            type="checkbox"
            class="history-checkbox"
            checked
            onchange={() => handleUncomplete(task.id)}
          />
          <span class="history-text">{task.text}</span>
          <span class="history-date">{formatDate(task.completed_at)}</span>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .history-view {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .history-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-xs) var(--space-md);
    border-bottom: 1px solid var(--color-border);
  }

  .back-button {
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    font-size: 18px;
    cursor: pointer;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--border-radius-sm);
    transition: color var(--transition-fast), background-color var(--transition-fast);
    line-height: 1;
  }

  .back-button:hover {
    color: var(--color-accent);
    background-color: var(--color-surface-hover);
  }

  .history-title {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm, 12px);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    color: var(--color-text-muted);
    font-size: var(--font-size-lg);
  }

  .history-list {
    list-style: none;
    overflow-y: auto;
    flex: 1;
  }

  .history-row {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid var(--color-border);
    transition: background-color var(--transition-fast);
  }

  .history-row:hover {
    background-color: var(--color-surface-hover);
  }

  .history-checkbox {
    appearance: none;
    width: 18px;
    height: 18px;
    min-width: 18px;
    border: 2px solid var(--color-accent);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    background-color: var(--color-accent);
    transition: background-color var(--transition-fast), border-color var(--transition-fast);
  }

  .history-checkbox:hover {
    opacity: 0.7;
  }

  .history-text {
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
    line-height: 1.4;
    word-break: break-word;
    flex: 1;
    text-decoration: line-through;
  }

  .history-date {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm, 12px);
    white-space: nowrap;
    flex-shrink: 0;
  }
</style>
