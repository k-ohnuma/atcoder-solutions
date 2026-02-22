import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import {
  createCommentBodySchema,
  deleteCommentQueryParams,
  updateCommentBodySchema,
} from "@/server/interface/solution/comment";
import { getCommentsBySolutionIdQueryParams } from "@/server/interface/solution/get";
import { authenticateRequest } from "@/server/utils/authRequest";

export async function GET(req: NextRequest) {
  const rawParams = Object.fromEntries(req.nextUrl.searchParams.entries());
  const parsed = getCommentsBySolutionIdQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const comments = await repo.getCommentsBySolutionId(parsed.data.solutionId);
  if (!comments.ok) {
    return NextResponse.json({ ok: false, error: comments.error }, { status: comments.status });
  }

  return NextResponse.json({ ok: true, data: comments.data }, { status: 200 });
}

export async function POST(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = createCommentBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const created = await repo.createComment(parsed.data.solutionId, parsed.data.bodyMd, authUser.data);
  if (!created.ok) {
    return NextResponse.json({ ok: false, error: created.error }, { status: created.status });
  }

  return NextResponse.json({ ok: true, data: created.data }, { status: 200 });
}

export async function PATCH(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = updateCommentBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const updated = await repo.updateComment(parsed.data.commentId, parsed.data.bodyMd, authUser.data);
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
  const parsed = deleteCommentQueryParams.safeParse(rawParams);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const repo = new SolutionRepositoryImpl();
  const deleted = await repo.deleteComment(parsed.data.commentId, authUser.data);
  if (!deleted.ok) {
    return NextResponse.json({ ok: false, error: deleted.error }, { status: deleted.status });
  }

  return NextResponse.json({ ok: true, data: deleted.data }, { status: 200 });
}
