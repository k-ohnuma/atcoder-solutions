import z from "zod";

const MAX_CONTEST_GROUP_QUERY_LENGTH = 100;
const MAX_CONTEST_GROUP_OFFSET = 5_000;

export const getProblemsByContestSeriesQueryParams = z.object({
  q: z.string().max(MAX_CONTEST_GROUP_QUERY_LENGTH).optional(),
  limit: z.coerce.number().int().positive().optional(),
  offset: z.coerce.number().int().nonnegative().max(MAX_CONTEST_GROUP_OFFSET).optional(),
});

export type GetProblemsByContestSeriesQueryParams = z.infer<typeof getProblemsByContestSeriesQueryParams>;
