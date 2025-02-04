/**
 * @file src/routes/+page.ts
 * @description +page
 * @author Tom Planche
 */
import type {PageLoad} from "../../.svelte-kit/types/src/routes/$types";
import axios from "axios";
import {TasksResponse, type TTasksResponse} from "$lib/types";

export const load: PageLoad = async ({fetch}) => {

  const tasks: TTasksResponse = await fetch('api/task/all').then(res => res.json());

  return {
    tasks
  };
}

/**
 * End of file src/routes/+page.ts
 */