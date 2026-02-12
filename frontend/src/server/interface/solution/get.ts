import z from "zod";

export const getSolutionBySolutionIdQueryParams = z.object({
  solutionId: z.string().uuid(),
});

export type GetSolutionBySolutionIdQueryParams = z.infer<typeof getSolutionBySolutionIdQueryParams>;

export const getSolutionsByProblemIdQueryParams = z.object({
  problemId: z.string().min(1),
});

export type GetSolutionsByProblemIdQueryParams = z.infer<typeof getSolutionsByProblemIdQueryParams>;
