import { NextRequest, NextResponse } from "next/server";
import { publicCacheHeaders } from "@/server/http/cache";
import { contestSeries } from "@/server/domain/problems";
import { ProblemRepositoryImpl } from "@/server/infrastructure/repository/problemRepository";
import { getProblemsByContestSeriesQueryParams } from "@/server/interface/problem/get";

type RouteContext = {
  params: Promise<{
    series: string;
  }>;
};

export async function GET(req: NextRequest, { params }: RouteContext) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsedQuery = getProblemsByContestSeriesQueryParams.safeParse(rawParams);
  if (!parsedQuery.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const { series: rawSeries } = await params;
  const parsedSeries = contestSeries.safeParse(rawSeries);
  if (!parsedSeries.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const { q, limit, offset } = parsedQuery.data;
  const repo = new ProblemRepositoryImpl();
  const problems = await repo.getContestGroupByContestSeries({
    series: parsedSeries.data,
    q,
    limit,
    offset,
  });
  if (!problems.ok) {
    return NextResponse.json({ ok: false, error: problems.error }, { status: problems.status });
  }
  const headers = q?.trim() ? { "Cache-Control": "no-store" } : publicCacheHeaders(3600, 300);
  return NextResponse.json({ ok: true, data: problems.data }, { status: 200, headers });
}
