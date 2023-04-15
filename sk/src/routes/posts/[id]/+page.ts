import { client } from "$lib/pocketbase";
import type { PostsResponse } from "$lib/pocketbase/pocketbase-types";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params: { id } }) => {
  const record = client.collection("posts").getOne<PostsResponse>(id);
  return {
    record,
  };
};
