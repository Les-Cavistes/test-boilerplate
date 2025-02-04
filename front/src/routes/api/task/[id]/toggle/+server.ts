/**
 * @file src/routes/api/task/[id]/toggle/+server.ts
 * @description +server
 * @author Tom Planche
 */

import {json, type RequestHandler} from '@sveltejs/kit';
import {PUBLIC_BACK_ENDPOINT} from '$env/static/public';
import {APIResponse} from '$lib/types/';
import {Task} from "$lib/types/task";

import axios from 'axios';
import {z} from "zod";

const ToggleTaskResponseSchema = APIResponse.extend({
  task: Task,
});

export type TToggleTaskResponse = z.infer<typeof ToggleTaskResponseSchema>;

export const PATCH: RequestHandler = async (event) => {
  const id = event.params.id;

  try {


    const response2 = ToggleTaskResponseSchema.parse(
      (await axios.patch(`${PUBLIC_BACK_ENDPOINT}/task/${id}/toggle`)).data
    );


    return json(response2);
  } catch (error) {
    return json(APIResponse.parse({
      completed: false,
      message: 'Failed to update task',
      status: 'error'
    }), {status: 500});
  }
};

/**
 * End of file src/routes/api/task/[id]/+server.ts
 */