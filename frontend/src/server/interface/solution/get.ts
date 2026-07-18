import z from "zod";

export const getSolutionsByProblemIdQueryParams = z.object({
  sortBy: z.enum(["latest", "votes"]).optional(),
  limit: z.coerce.number().int().positive().optional(),
});

export type GetSolutionsByProblemIdQueryParams = z.infer<typeof getSolutionsByProblemIdQueryParams>;

export const getSolutionsByUserNameQueryParams = z.object({
  sortBy: z.enum(["latest", "votes"]).optional(),
});

export type GetSolutionsByUserNameQueryParams = z.infer<typeof getSolutionsByUserNameQueryParams>;

export const getLatestSolutionsQueryParams = z.object({
  sortBy: z.literal("latest").optional(),
  limit: z.coerce.number().int().positive().optional(),
});

export type GetLatestSolutionsQueryParams = z.infer<typeof getLatestSolutionsQueryParams>;
