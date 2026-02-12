import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getSolutionVotesCountQueryParams } from "@/server/interface/solution/get";
import { unvoteSolutionQueryParams, voteSolutionBodySchema } from "@/server/interface/solution/vote";
import { authenticateRequest } from "@/server/utils/authRequest";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getSolutionVotesCountQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const count = await repo.getVotesCount(parsed.data.solutionId);
  if (!count.ok) {
    return NextResponse.json({ ok: false, error: count.error }, { status: count.status });
  }

  return NextResponse.json({ ok: true, data: count.data }, { status: 200 });
}

export async function POST(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = voteSolutionBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const voted = await repo.vote(parsed.data.solutionId, authUser.data);
  if (!voted.ok) {
    return NextResponse.json({ ok: false, error: voted.error }, { status: voted.status });
  }

  return NextResponse.json({ ok: true, data: voted.data }, { status: 200 });
}

export async function DELETE(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = unvoteSolutionQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const unvoted = await repo.unvote(parsed.data.solutionId, authUser.data);
  if (!unvoted.ok) {
    return NextResponse.json({ ok: false, error: unvoted.error }, { status: unvoted.status });
  }

  return NextResponse.json({ ok: true, data: unvoted.data }, { status: 200 });
}
