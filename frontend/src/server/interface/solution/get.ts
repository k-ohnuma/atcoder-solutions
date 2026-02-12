import z from "zod";

export const getSolutionBySolutionIdQueryParams = z.object({
  solutionId: z.string(),
});

export type GetSolutionBySolutionIdQueryParams = z.infer<typeof getSolutionBySolutionIdQueryParams>;

export const getSolutionsByProblemIdQueryParams = z.object({
  problemId: z.string().min(1),
  sortBy: z.enum(["latest", "votes"]).optional(),
});

export type GetSolutionsByProblemIdQueryParams = z.infer<typeof getSolutionsByProblemIdQueryParams>;

export const getSolutionVotesCountQueryParams = z.object({
  solutionId: z.string(),
});

export type GetSolutionVotesCountQueryParams = z.infer<typeof getSolutionVotesCountQueryParams>;

export const getMySolutionVoteStatusQueryParams = z.object({
  solutionId: z.string(),
});

export type GetMySolutionVoteStatusQueryParams = z.infer<typeof getMySolutionVoteStatusQueryParams>;
