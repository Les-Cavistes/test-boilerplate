/**
 * @file src/routes/+page.ts
 * @description +page
 * @author Tom Planche
 */
import type { PageLoad } from "../../.svelte-kit/types/src/routes/$types";
import { tasksStore } from "$lib/stores/tasks";
import type { TTasksResponse } from "$lib/types/task";

export const load: PageLoad = async ({ fetch }) => {
    try {
        const tasks: TTasksResponse = await fetch("api/task/all").then((res) =>
            res.json(),
        );

        if (!tasks.tasks) return;

        // Initialize the store with the loaded data
        tasksStore.set(tasks);
    } catch (error) {
        // console.error("Error loading tasks:", error);
    }
};

/**
 * End of file src/routes/+page.ts
 */
