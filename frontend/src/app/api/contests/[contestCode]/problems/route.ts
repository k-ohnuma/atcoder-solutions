import { NextRequest, NextResponse } from "next/server";
import { publicCacheHeaders } from "@/server/http/cache";
import { ProblemRepositoryImpl } from "@/server/infrastructure/repository/problemRepository";

type RouteContext = {
  params: Promise<{
    contestCode: string;
  }>;
};

export async function GET(_req: NextRequest, { params }: RouteContext) {
  const { contestCode } = await params;
  const normalizedContestCode = contestCode.trim();
  if (!normalizedContestCode) {
    return NextResponse.json({ ok: false, error: "contestCode cannot be empty" }, { status: 400 });
  }

  const repo = new ProblemRepositoryImpl();
  const problems = await repo.getProblemsByContest(normalizedContestCode);
  if (!problems.ok) {
    return NextResponse.json({ ok: false, error: problems.error }, { status: problems.status });
  }

  return NextResponse.json({ ok: true, data: problems.data }, { status: 200, headers: publicCacheHeaders(3600, 300) });
}
