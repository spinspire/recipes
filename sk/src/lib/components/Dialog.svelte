<script lang="ts">
  let dialog: HTMLDialogElement;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<span on:click={(e) => dialog.showModal()}>
  <slot name="trigger">
    <button>Open Dialog</button>
  </slot>
</span>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog bind:this={dialog} on:click={(e) => e.target?.close()}>
  <slot />
</dialog>
