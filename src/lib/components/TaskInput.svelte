<script lang="ts">
  import { createTask } from "../stores/taskStore";

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();
  let text = $state("");
  let inputEl: HTMLInputElement | undefined = $state();

  $effect(() => {
    if (inputEl) {
      inputEl.focus();
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && text.trim()) {
      e.preventDefault();
      createTask(text.trim());
      text = "";
    } else if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      onclose();
    }
  }
</script>

<div class="task-input">
  <input
    bind:this={inputEl}
    bind:value={text}
    onkeydown={handleKeydown}
    type="text"
    placeholder="Nouvelle tâche..."
    class="input-field"
  />
</div>

<style>
  .task-input {
    padding: var(--space-sm) var(--space-md);
    border-bottom: 2px solid var(--color-accent);
  }

  .input-field {
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    color: var(--color-text-primary);
    font-family: var(--font-family);
    font-size: var(--font-size-base);
    padding: var(--space-xs) 0;
  }

  .input-field::placeholder {
    color: var(--color-text-muted);
  }
</style>
