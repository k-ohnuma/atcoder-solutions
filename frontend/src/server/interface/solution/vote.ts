import z from "zod";

export const voteSolutionBodySchema = z.object({
  solutionId: z.string(),
});

export const unvoteSolutionQueryParams = z.object({
  solutionId: z.string(),
});
