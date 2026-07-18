import z from "zod";

export const getSolutionsByProblemIdQueryParams = z.object({
  problemId: z.string().min(1),
  sortBy: z.enum(["latest", "votes"]).optional(),
  size: z.coerce.number().int().positive().optional(),
});

export type GetSolutionsByProblemIdQueryParams = z.infer<typeof getSolutionsByProblemIdQueryParams>;

export const getLatestSolutionsQueryParams = z.object({
  sortBy: z.literal("latest").optional(),
  size: z.coerce.number().int().positive().optional(),
});

export type GetLatestSolutionsQueryParams = z.infer<typeof getLatestSolutionsQueryParams>;
