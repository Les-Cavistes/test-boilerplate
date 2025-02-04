/**
 * @file src/routes/api/task/[id]/+server.ts
 * @description +server
 * @author Tom Planche
 */

import {json, type RequestHandler} from '@sveltejs/kit';
import axios from 'axios';
import { PUBLIC_BACK_ENDPOINT } from '$env/static/public';

export const GET: RequestHandler = async (event) => {
  const id = event.params.id;

  try {
    const response = await axios.get(`${PUBLIC_BACK_ENDPOINT}/task/${id}/toggle`);
    
    return json({
      completed: response.data.completed,
      message: `Successfully toggled task ${id}`,
      status: 'success'
    }, { status: 200 });
  } catch (error) {
    return json({
      completed: false,
      message: 'Failed to update task',
      status: 'error'
    }, { status: 500 });
  }
};

/**
 * End of file src/routes/api/task/[id]/+server.ts
 */