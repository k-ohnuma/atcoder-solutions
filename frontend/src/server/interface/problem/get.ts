import { contestSeries } from "@/server/domain/problems";
import z from "zod";

export const getProblemsByContestSeriesQueryParams = z.object({
  series: contestSeries,
});

export type GetProblemsByContestSeriesQueryParams = z.infer<typeof getProblemsByContestSeriesQueryParams>;
