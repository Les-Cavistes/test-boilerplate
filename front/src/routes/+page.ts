/**
 * @file src/routes/+page.ts
 * @description +page
 * @author Tom Planche
 */
import type {PageLoad} from "../../.svelte-kit/types/src/routes/$types";
import { tasksStore } from '$lib/stores/tasks';
import type {TTasksResponse} from "$lib/types/task";

export const load: PageLoad = async ({fetch}) => {
  const tasks: TTasksResponse = await fetch('api/task/all').then(res => res.json());

  // Initialize the store with the loaded data
  tasksStore.set(tasks);
}

/**
 * End of file src/routes/+page.ts
 */