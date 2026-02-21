import z from "zod";

export const createCommentBodySchema = z.object({
  solutionId: z.string(),
  bodyMd: z.string().trim().min(1).max(2000),
});
