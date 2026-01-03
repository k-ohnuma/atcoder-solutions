import z from "zod";

export const createSolutionBodySchema = z.object({
  problemId: z.string(),
  title: z.string(),
  bodyMd: z.string(),
  submitUrl: z.string(),
  tags: z.array(z.string()),
});
