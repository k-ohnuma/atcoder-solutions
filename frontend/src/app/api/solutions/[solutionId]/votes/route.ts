import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";

type RouteContext = {
  params: Promise<{
    solutionId: string;
  }>;
};

export async function GET(_req: NextRequest, { params }: RouteContext) {
  const { solutionId } = await params;
  const repo = new SolutionRepositoryImpl();
  const count = await repo.getVotesCount(solutionId);
  if (!count.ok) {
    return NextResponse.json({ ok: false, error: count.error }, { status: count.status });
  }

  return NextResponse.json({ ok: true, data: count.data }, { status: 200 });
}
