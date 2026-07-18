import { NextRequest, NextResponse } from "next/server";
import { SolutionRepositoryImpl } from "@/server/infrastructure/repository/solutionRepository";
import { updateCommentBodySchema } from "@/server/interface/solution/comment";
import { authenticateRequest } from "@/server/utils/authRequest";

type RouteContext = {
  params: Promise<{
    commentId: string;
  }>;
};

export async function PATCH(req: NextRequest, { params }: RouteContext) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const body = await req.json();
  const parsed = updateCommentBodySchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ ok: false, error: "invalid format" }, { status: 400 });
  }

  const { commentId } = await params;
  const repo = new SolutionRepositoryImpl();
  const updated = await repo.updateComment(commentId, parsed.data.bodyMd, authUser.data);
  if (!updated.ok) {
    return NextResponse.json({ ok: false, error: updated.error }, { status: updated.status });
  }

  return NextResponse.json({ ok: true, data: updated.data }, { status: 200 });
}

export async function DELETE(req: NextRequest, { params }: RouteContext) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const { commentId } = await params;
  const repo = new SolutionRepositoryImpl();
  const deleted = await repo.deleteComment(commentId, authUser.data);
  if (!deleted.ok) {
    return NextResponse.json({ ok: false, error: deleted.error }, { status: deleted.status });
  }

  return NextResponse.json({ ok: true, data: deleted.data }, { status: 200 });
}
