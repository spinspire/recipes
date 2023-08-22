<script lang="ts">
  export let authCollection = "users";
  export let passwordLogin = true;
  import { client, providerLogin } from "../pocketbase";
  const coll = client.collection(authCollection);

  let username: string;
  let password: string;

  async function submit() {
    await coll.authWithPassword(username, password);
  }
</script>

<form on:submit|preventDefault={submit}>
  {#if passwordLogin}
    <input bind:value={username} required type="text" placeholder="username / email" />
    <input bind:value={password} required type="password" placeholder="password" />
    <button type="submit">Sign In</button>
  {/if}
  {#await coll.listAuthMethods({ $autoCancel: false }) then methods}
    {#each methods.authProviders as p}
      <button type="button" on:click={() => providerLogin(p, coll)}>Login with {p.name}</button>
    {/each}
  {:catch}
    <!-- pocketbase not working -->
  {/await}
</form>
