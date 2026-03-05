<script lang="ts">
  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();
</script>

<div class="settings-view">
  <div class="settings-header">
    <button class="back-button" onclick={onclose} title="Retour">←</button>
    <span class="settings-title">Paramètres</span>
  </div>

  <div class="settings-content">
    <p class="placeholder">Paramètres à venir...</p>
  </div>
</div>

<style>
  .settings-view {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .settings-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
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

  .settings-title {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm, 12px);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .settings-content {
    flex: 1;
    padding: var(--space-lg) var(--space-md);
    overflow-y: auto;
  }

  .placeholder {
    color: var(--color-text-muted);
    text-align: center;
    padding: var(--space-xl);
  }
</style>
