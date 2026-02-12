import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getSolutionsByProblemIdQueryParams } from "@/server/interface/solution/get";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getSolutionsByProblemIdQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const solutions = await repo.getByProblemId(parsed.data.problemId, parsed.data.sortBy);
  if (!solutions.ok) {
    return NextResponse.json({ ok: false, error: solutions.error }, { status: solutions.status });
  }

  return NextResponse.json({ ok: true, data: solutions.data }, { status: 200 });
}
