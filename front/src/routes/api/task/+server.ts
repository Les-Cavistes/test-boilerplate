import {json} from '@sveltejs/kit';
import type {RequestHandler} from './$types';
import {PUBLIC_BACK_ENDPOINT} from '$env/static/public';
import {z} from 'zod';
import axios from 'axios';
import {APIResponse} from "$lib/types";

const createTaskSchema = z.object({
  description: z.string().min(1, "Description cannot be empty")
});

export const POST: RequestHandler = async ({request}) => {
  try {
    // Parse and validate request body
    const body = await request.json();
    const {description} = createTaskSchema.parse(body);

    // Forward the request to the backend using axios
    const {data} = await axios.post(`${PUBLIC_BACK_ENDPOINT}/task`, {
      description
    });

    return json(data);
  } catch (error) {
    console.error('Error creating task:', error);

    if (error instanceof z.ZodError) {
      return json({
        status: 'error',
        message: `Invalid request data: ${error.errors[0].message}`
      }, {status: 400});
    }

    // Handle axios errors
    if (axios.isAxiosError(error)) {
      return json({
        status: 'error',
        message: error.response?.data?.message || 'Backend server error'
      }, {status: error.response?.status || 500});
    }

    return json(
      APIResponse.parse({
        message: 'Failed to create task',
        status: 'error',
        error: (error as Error).message
      }), {status: 500});
  }
}; 