/**
 * @file src/routes/api/task/[id]/+server.ts
 * @description +server
 * @author Tom Planche
 */
import {json, type RequestHandler} from "@sveltejs/kit";
import axios from "axios";
import {PUBLIC_BACK_ENDPOINT} from "$env/static/public";
import {APIResponse} from "$lib/types";

export const DELETE: RequestHandler = async (event) => {
  const id = event.params.id;

  try {
    await axios.delete(`${PUBLIC_BACK_ENDPOINT}/task/${id}`);

    return json(
      {
        message: `Successfully deleted task ${id}`,
        status: "success",
      },
      {status: 200},
    );
  } catch (error) {
    return json(
      APIResponse.parse({
        message: "Failed to delete task",
        status: "error",
        error
      }), {status: 500}
    );

  }
};

/**
 * End of file src/routes/api/task/[id]/+server.ts
 */
