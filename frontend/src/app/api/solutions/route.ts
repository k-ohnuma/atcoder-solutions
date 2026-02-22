import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { getSolutionBySolutionIdQueryParams } from "@/server/interface/solution/get";
import {
  createSolutionBodySchema,
  deleteSolutionQueryParams,
  updateSolutionBodySchema,
} from "@/server/interface/solution/post";
import { authenticateRequest } from "@/server/utils/authRequest";

export async function POST(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = createSolutionBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const solution = parsed.data;
  const repo = new SolutionRepositoryImpl();
  const id = await repo.create(solution, authUser.data);
  if (!id.ok) {
    return NextResponse.json({ ok: false, error: id.error }, { status: id.status });
  }
  return NextResponse.json({ ok: true, data: id.data }, { status: 200 });
}

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getSolutionBySolutionIdQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const solution = await repo.getBySolutionId(parsed.data.solutionId);
  if (!solution.ok) {
    return NextResponse.json({ ok: false, error: solution.error }, { status: solution.status });
  }
  return NextResponse.json({ ok: true, data: solution.data }, { status: 200 });
}

export async function PATCH(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = updateSolutionBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const updated = await repo.update(parsed.data, authUser.data);
  if (!updated.ok) {
    return NextResponse.json({ ok: false, error: updated.error }, { status: updated.status });
  }
  return NextResponse.json({ ok: true, data: updated.data }, { status: 200 });
}

export async function DELETE(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = deleteSolutionQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const deleted = await repo.delete(parsed.data.solutionId, authUser.data);
  if (!deleted.ok) {
    return NextResponse.json({ ok: false, error: deleted.error }, { status: deleted.status });
  }
  return NextResponse.json({ ok: true, data: deleted.data }, { status: 200 });
}
