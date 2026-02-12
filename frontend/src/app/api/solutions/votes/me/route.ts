import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getMySolutionVoteStatusQueryParams } from "@/server/interface/solution/get";
import { authenticateRequest } from "@/server/utils/authRequest";

export async function GET(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getMySolutionVoteStatusQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const status = await repo.getMyVoteStatus(parsed.data.solutionId, authUser.data);
  if (!status.ok) {
    return NextResponse.json({ ok: false, error: status.error }, { status: status.status });
  }

  return NextResponse.json({ ok: true, data: status.data }, { status: 200 });
}
