import z from "zod";

export const createCommentBodySchema = z.object({
  bodyMd: z.string().trim().min(1).max(2000),
});

export const updateCommentBodySchema = z.object({
  bodyMd: z.string().trim().min(1).max(2000),
});
