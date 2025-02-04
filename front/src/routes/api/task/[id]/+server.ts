/**
 * @file src/routes/api/task/[id]/+server.ts
 * @description +server
 * @author Tom Planche
 */

import {json} from '@sveltejs/kit';
import type {RequestHandler} from './$types';
import axios from 'axios';
import { PUBLIC_BACK_ENDPOINT } from '$env/static/public';

export const PUT: RequestHandler = async (event) => {
  const id = event.params.id;

  try {
    const response = await axios.put(`${PUBLIC_BACK_ENDPOINT}/task/${id}`);
    
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