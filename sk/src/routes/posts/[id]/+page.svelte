<script lang="ts">
  import { client } from "$lib/pocketbase";
  import ImgModal from "$lib/pocketbase/ImgModal.svelte";
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<h1>{data.record.title}</h1>
<article>
  <ImgModal record={data.record} filename={data.record.files[0]} />
  <div>{@html data.record.body}</div>
</article>

<style lang="scss">
  h1 {
    border-bottom: dashed 2px;
  }
  article :global(img) {
    float: right;
  }
</style>
