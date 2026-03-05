<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { enable, disable } from "@tauri-apps/plugin-autostart";
  import { config, loadConfig, saveConfig } from "../stores/configStore";
  import type { Config } from "../types";

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();
  let capturing = $state(false);
  let capturedKeys = $state("");

  // Load config on mount
  loadConfig();

  async function toggleShortcutMode() {
    const current = $config;
    const newMode = current.shortcut_mode === "toggle" ? "hold" : "toggle";
    const newConfig: Config = { ...current, shortcut_mode: newMode };
    await saveConfig(newConfig);
  }

  function startCapture() {
    capturing = true;
    capturedKeys = "Appuyez sur une combinaison...";
  }

  function handleCaptureKeydown(e: KeyboardEvent) {
    if (!capturing) return;
    e.preventDefault();
    e.stopPropagation();

    // Build modifier string
    const parts: string[] = [];
    if (e.ctrlKey || e.metaKey) parts.push("CmdOrCtrl");
    if (e.altKey) parts.push("Alt");
    if (e.shiftKey) parts.push("Shift");

    // Only accept if there's a non-modifier key
    const key = e.key;
    const isModifier = ["Control", "Meta", "Alt", "Shift"].includes(key);
    if (isModifier) {
      capturedKeys = parts.join("+") + "+...";
      return;
    }

    // Map common keys
    let keyName = key;
    if (key === " ") keyName = "Space";
    else if (key.length === 1) keyName = key.toUpperCase();

    parts.push(keyName);
    const shortcutStr = parts.join("+");

    applyShortcut(shortcutStr);
  }

  async function applyShortcut(shortcutStr: string) {
    capturing = false;
    try {
      await invoke("update_shortcut", { shortcut: shortcutStr });
      const newConfig: Config = { ...$config, shortcut: shortcutStr };
      await saveConfig(newConfig);
    } catch (err) {
      capturedKeys = `Erreur: ${err}`;
    }
  }

  function cancelCapture() {
    capturing = false;
    capturedKeys = "";
  }

  async function toggleAutostart() {
    const current = $config;
    if (current.autostart) {
      await disable();
    } else {
      await enable();
    }
    const newConfig: Config = { ...current, autostart: !current.autostart };
    await saveConfig(newConfig);
  }
</script>

<svelte:window onkeydown={handleCaptureKeydown} />

<div class="settings-view">
  <div class="settings-header">
    <button class="back-button" onclick={onclose} title="Retour">←</button>
    <span class="settings-title">Paramètres</span>
  </div>

  <div class="settings-content">
    <!-- Shortcut Section -->
    <div class="setting-section">
      <h3 class="section-title">Raccourci global</h3>

      <div class="setting-row">
        <span class="setting-label">Raccourci actuel</span>
        <span class="setting-value shortcut-display">{$config.shortcut}</span>
      </div>

      <div class="setting-row">
        {#if capturing}
          <span class="setting-label capture-hint">{capturedKeys}</span>
          <button class="setting-button" onclick={cancelCapture}>Annuler</button>
        {:else}
          <span class="setting-label">Modifier le raccourci</span>
          <button class="setting-button" onclick={startCapture}>Capturer</button>
        {/if}
      </div>

      <div class="setting-row">
        <span class="setting-label">Mode</span>
        <button class="setting-button mode-toggle" onclick={toggleShortcutMode}>
          {$config.shortcut_mode === "toggle" ? "Toggle" : "Maintenu"}
        </button>
      </div>
    </div>

    <!-- Autostart Section -->
    <div class="setting-section">
      <h3 class="section-title">Système</h3>

      <div class="setting-row">
        <span class="setting-label">Lancer au démarrage</span>
        <button
          class="setting-button mode-toggle"
          class:active={$config.autostart}
          onclick={toggleAutostart}
        >
          {$config.autostart ? "Activé" : "Désactivé"}
        </button>
      </div>
    </div>
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

  .setting-section {
    margin-bottom: var(--space-xl);
  }

  .section-title {
    color: var(--color-text-primary);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    margin-bottom: var(--space-md);
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) 0;
    border-bottom: 1px solid var(--color-border);
  }

  .setting-label {
    color: var(--color-text-secondary);
    font-size: var(--font-size-base);
  }

  .setting-value {
    color: var(--color-text-primary);
    font-size: var(--font-size-base);
  }

  .shortcut-display {
    font-family: monospace;
    background: var(--color-surface-hover);
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--border-radius-sm);
  }

  .capture-hint {
    color: var(--color-accent);
    font-size: var(--font-size-sm, 12px);
  }

  .setting-button {
    background: var(--color-surface-hover);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm, 12px);
    cursor: pointer;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--border-radius-sm);
    transition: background-color var(--transition-fast), border-color var(--transition-fast);
  }

  .setting-button:hover {
    background-color: var(--color-surface);
    border-color: var(--color-accent);
  }

  .mode-toggle {
    min-width: 80px;
    text-align: center;
  }

  .mode-toggle.active {
    background-color: var(--color-accent);
    border-color: var(--color-accent);
    color: #fff;
  }
</style>
