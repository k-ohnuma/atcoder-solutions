import z from "zod";

export const getSolutionBySolutionIdQueryParams = z.object({
  solutionId: z.string().uuid(),
});

export type GetSolutionBySolutionIdQueryParams = z.infer<typeof getSolutionBySolutionIdQueryParams>;
