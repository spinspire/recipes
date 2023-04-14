<script lang="ts">
  import { page } from "$app/stores";
  import type { AuthProviderInfo } from "pocketbase";

  export let authCollection = "users";
  export let passwordLogin = true;
  const coll = client.collection(authCollection);

  import { client, handleRedirect } from "../pocketbase";
  import { onMount } from "svelte";

  let username: string;
  let password: string;

  // IMPORTANT: This handles the incoming OAuth2 redirect from auth provider
  onMount(() => handleRedirect($page));

  async function submit() {
    await coll.authWithPassword(username, password);
  }
  function save(p: AuthProviderInfo) {
    sessionStorage.setItem("provider", JSON.stringify(p));
    sessionStorage.setItem("redirect", $page.url.toString());
  }
</script>

<form on:submit|preventDefault={submit}>
  {#if passwordLogin}
    <input bind:value={username} type="text" placeholder="username / email" />
    <input bind:value={password} type="password" placeholder="password" />
    <button type="submit">Sign In</button>
  {/if}
  {#await coll.listAuthMethods({ $autoCancel: false }) then methods}
    {@const redirectUri = new URL("/", $page.url).toString()}
    {#each methods.authProviders as p}
      <button>
        <a href={p.authUrl + redirectUri} on:click={() => save(p)}>Login with {p.name}</a>
      </button>
    {/each}
  {:catch}
    <!-- pocketbase not working -->
  {/await}
</form>

<style>
  button > a {
    font-weight: bold;
  }
</style>
