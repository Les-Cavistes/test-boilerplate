/**
 * @file src/routes/api/all/+page.ts
 * @description +page
 * @author Tom Planche
 */
import axios from "axios";
import {PUBLIC_BACK_ENDPOINT} from "$env/static/public";
import type {RequestHandler} from "@sveltejs/kit";
import {json} from "@sveltejs/kit";
import {TasksResponse, type TTasksResponse} from "$lib/types/task";
import {APIResponse} from "$lib/types";

export const GET: RequestHandler = async () => {
    try {
        const res = await axios.get(`${PUBLIC_BACK_ENDPOINT}/task/all`);

        const tasks: TTasksResponse = TasksResponse.parse(res.data.data);

        return json(tasks);
    } catch (error) {
        console.error("Error loading tasks:", error);
        return json(
          APIResponse.parse({
              message: "Failed to load tasks",
              status: "error",
              error
          }), {status: 500}
        );
    }
};

/**
 * End of file src/routes/api/all/+page.ts
 */
