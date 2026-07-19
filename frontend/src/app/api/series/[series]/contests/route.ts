import { NextRequest, NextResponse } from "next/server";
import { contestSeries } from "@/server/domain/problems";
import { publicCacheHeaders } from "@/server/http/cache";
import { ContestRepositoryImpl } from "@/server/infrastructure/repository/contestRepository";

type RouteContext = {
  params: Promise<{
    series: string;
  }>;
};

export async function GET(_req: NextRequest, { params }: RouteContext) {
  const { series } = await params;
  const parsed = contestSeries.safeParse(series);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid series" }, { status: 400 });
  }

  const repo = new ContestRepositoryImpl();
  const contests = await repo.getContestsBySeries(parsed.data);
  if (!contests.ok) {
    return NextResponse.json({ ok: false, error: contests.error }, { status: contests.status });
  }

  return NextResponse.json({ ok: true, data: contests.data }, { status: 200, headers: publicCacheHeaders(3600, 300) });
}
