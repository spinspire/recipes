<script lang="ts">
  import { authModel, client } from "../pocketbase";
  import Dialog from "./Dialog.svelte";
  import LoginForm from "./LoginForm.svelte";
  async function logout() {
    client.authStore.clear();
  }
</script>

{#if $authModel}
  <details>
    <summary>
      {#if $authModel.avatar}
        <img src={client.getFileUrl($authModel, $authModel.avatar)} alt="profile pic" />
      {/if}
      <code>{$authModel?.name ?? $authModel?.username ?? $authModel?.email}</code>
    </summary>
    <button on:click={logout}>Sign Out</button>
  </details>
{:else}
  <Dialog>
    <button slot="trigger">Sign In</button>
    <h2><center>Sign In</center></h2>
    <LoginForm />
  </Dialog>
{/if}

<style lang="scss">
  summary {
    display: flex;
    align-items: center;
    > img {
      height: 2em;
      width: 2em;
      border-radius: 50%;
    }
  }
</style>
