import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getSolutionsByProblemIdQueryParams } from "@/server/interface/solution/get";

type RouteContext = {
  params: Promise<{
    problemId: string;
  }>;
};

export async function GET(req: NextRequest, { params }: RouteContext) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getSolutionsByProblemIdQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const { problemId } = await params;
  const repo = new SolutionRepositoryImpl();
  const solutions = await repo.getByProblemId(problemId, parsed.data.sortBy, parsed.data.limit);
  if (!solutions.ok) {
    return NextResponse.json({ ok: false, error: solutions.error }, { status: solutions.status });
  }

  return NextResponse.json({ ok: true, data: solutions.data }, { status: 200 });
}
