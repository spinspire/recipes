<script lang="ts" context="module">
  import { writable } from "svelte/store";

  // returns a store that:
  // starts out false
  // becomes true when the async function f starts running
  // becomes false when f resolves (or rejects)
  export function activityStore(f: () => {}) {
    const store = writable(false);
    async function run() {
      try {
        store.set(true);
        return await f();
      } finally {
        store.set(false);
      }
    }
    return { ...store, run };
  }
</script>

<script lang="ts">
  export let active = false;
</script>

<span class="loader" class:active />

<style lang="scss">
  .loader {
    width: 1em;
    height: 1em;
    border: 0.2em solid var(--variable);
    border-radius: 50%;
    display: inline-block;
    box-sizing: border-box;
    &.active {
      border-bottom-color: transparent;
      animation: rotation 1s linear infinite;
    }
  }

  @keyframes rotation {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
