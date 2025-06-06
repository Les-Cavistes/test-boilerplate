/**
 * @file src/routes/api/task/types.ts
 * @description types
 * @author Tom Planche
 */
import {z} from "zod";

export const Task = z.object({
  id: z.number(),
  description: z.string(),
  completed: z.boolean(),
});

export type TTask = z.infer<typeof Task>;

export const TasksResponse = z.object({
  tasks: z.array(Task),
  total: z.number(),
  page: z.number(),
  per_page: z.number(),
  total_pages: z.number(),
});

export type TTasksResponse = z.infer<typeof TasksResponse>;

/**
 * End of file src/routes/api/task/types.ts
 */