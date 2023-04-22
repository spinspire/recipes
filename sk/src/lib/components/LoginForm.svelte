<script lang="ts">
  import { page } from "$app/stores";
  import type { AuthProviderInfo, RecordService } from "pocketbase";

  export let authCollection = "users";
  export let passwordLogin = true;
  const coll = client.collection(authCollection);

  import { client, providerLogin } from "../pocketbase";
  import { alerts } from "./Alerts.svelte";
  import { onDestroy, onMount } from "svelte";

  let username: string;
  let password: string;

  async function submit() {
    await coll.authWithPassword(username, password);
  }
  async function oauth(p: AuthProviderInfo) {
    const result = await providerLogin(p, coll);
    // console.log({ result });
  }
  const unsubscribe = client.authStore.onChange((token, model: any) => {
    if (model) {
      const { name, username } = model;
      alerts.success(`Signed in as ${name || username}`, 5000);
    } else {
      alerts.success(`Signed out`, 5000);
    }
  });
  onDestroy(() => {
    unsubscribe();
  });
</script>

<form on:submit|preventDefault={submit}>
  {#if passwordLogin}
    <input bind:value={username} required type="text" placeholder="username / email" />
    <input bind:value={password} required type="password" placeholder="password" />
    <button type="submit">Sign In</button>
  {/if}
  {#await coll.listAuthMethods({ $autoCancel: false }) then methods}
    {@const redirectUri = new URL("/", $page.url).toString()}
    {#each methods.authProviders as p}
      <button type="button" on:click={() => oauth(p)}>Login with {p.name}</button>
    {/each}
  {:catch}
    <!-- pocketbase not working -->
  {/await}
</form>
