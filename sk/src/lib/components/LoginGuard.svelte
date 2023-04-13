<script>
  import { authModel } from "../pocketbase";
  import LoginForm from "./LoginForm.svelte";
</script>

{#if $authModel}
  <slot />
{:else}
  <slot name="anonymous">
    <LoginForm />
  </slot>
{/if}
