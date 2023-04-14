<script lang="ts">
  import { authModel, client } from "../pocketbase";
  import Dialog from "./Dialog.svelte";
  import LoginForm from "./LoginForm.svelte";
  async function logout() {
    client.authStore.clear();
  }
</script>

{#if $authModel}
  <Dialog>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="badge" slot="trigger" on:click={() => console.log($authModel)}>
      {#if $authModel.avatar}
        <img src={client.getFileUrl($authModel, $authModel.avatar)} alt="profile pic" />
      {/if}
      <samp>{$authModel?.name ?? $authModel?.username ?? $authModel?.email}</samp>
    </div>
    <div class="wrapper">
      <div class="badge">
        {#if $authModel.avatar}
          <img src={client.getFileUrl($authModel, $authModel.avatar)} alt="profile pic" />
        {/if}
        <samp>{$authModel?.name ?? $authModel?.username ?? $authModel?.email}</samp>
      </div>
      <button on:click={logout}>Sign Out</button>
    </div>
  </Dialog>
{:else}
  <Dialog>
    <button slot="trigger">Sign In</button>
    <h2><center>Sign In</center></h2>
    <LoginForm />
  </Dialog>
{/if}

<style lang="scss">
  .badge {
    cursor: pointer;
    display: flex;
    align-items: center;
    padding: 5px;
    gap: 5px;
    > img {
      height: 2em;
      width: 2em;
      border-radius: 50%;
    }
  }
  .wrapper {
    display: flex;
    flex-direction: column;
  }
</style>
