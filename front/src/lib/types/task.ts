/**
 * @file src/lib/types/task.ts
 * @description task
 * @author Tom Planche
 */

import {z} from "zod";

export const Task = z.object({
  id: z.number(),
  description: z.string(),
  completed: z.boolean(),
});

export type TTask = z.infer<typeof Task>;

const PaginationSchema = z.object({
  current_page: z.number(),
  per_page: z.number(),
  total_items: z.number(),
  total_pages: z.number(),
});

export const TasksResponse = z.object({
  tasks: z.array(Task),
  pagination: PaginationSchema,
});

export type TTasksResponse = z.infer<typeof TasksResponse>;

/**
 * End of file src/lib/types/task.ts
 */