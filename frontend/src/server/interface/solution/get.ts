import z from "zod";

export const getSolutionBySolutionIdQueryParams = z.object({
  solutionId: z.string(),
});

export type GetSolutionBySolutionIdQueryParams = z.infer<typeof getSolutionBySolutionIdQueryParams>;

export const getSolutionsByProblemIdQueryParams = z.object({
  problemId: z.string().min(1),
  sortBy: z.enum(["latest", "votes"]).optional(),
  size: z.coerce.number().int().positive().optional(),
});

export type GetSolutionsByProblemIdQueryParams = z.infer<typeof getSolutionsByProblemIdQueryParams>;

export const getLatestSolutionsQueryParams = z.object({
  size: z.coerce.number().int().positive().optional(),
});

export type GetLatestSolutionsQueryParams = z.infer<typeof getLatestSolutionsQueryParams>;

export const getSolutionVotesCountQueryParams = z.object({
  solutionId: z.string(),
});

export type GetSolutionVotesCountQueryParams = z.infer<typeof getSolutionVotesCountQueryParams>;

export const getMySolutionVoteStatusQueryParams = z.object({
  solutionId: z.string(),
});

export type GetMySolutionVoteStatusQueryParams = z.infer<typeof getMySolutionVoteStatusQueryParams>;

export const getCommentsBySolutionIdQueryParams = z.object({
  solutionId: z.string(),
});

export type GetCommentsBySolutionIdQueryParams = z.infer<typeof getCommentsBySolutionIdQueryParams>;
