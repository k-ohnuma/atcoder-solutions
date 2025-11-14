import z from "zod";
import { contestSeries } from "@/server/domain/problems";

export const getProblemsByContestSeriesQueryParams = z.object({
  series: contestSeries,
});

export type GetProblemsByContestSeriesQueryParams = z.infer<typeof getProblemsByContestSeriesQueryParams>;
