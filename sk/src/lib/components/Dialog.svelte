<script lang="ts">
  export let title = "";
  let dialog: HTMLDialogElement;
  const C = {
    close(e: any) {
      if (typeof e?.target?.close === "function") e.target.close();
    },
    open(e: Event) {
      dialog.showModal();
    },
  };
</script>

<button on:click={C.open}>
  <slot name="trigger" {...C}>Open</slot>
</button>
<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog bind:this={dialog} on:click={C.close}>
  <slot name="header" {...C}>
    {#if title}
      <header><h3>{title}</h3></header>
    {/if}
  </slot>
  <slot {...C} />
</dialog>
