/**
 * @file src/lib/types.d.ts
 * @description types.d
 * @author Tom Planche
 */
import {z} from "zod";

export const APIResponse = z.object({
  status: z.string(),
  message: z.string(),
});



/**
 * End of file src/lib/types.d.ts
 */