import { NextRequest, NextResponse } from "next/server";
import { UserRepositoryImpl } from "@/server/infrastructure/repository/userRepository";
import { authenticateRequest } from "@/server/utils/authRequest";

export async function GET(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const repo = new UserRepositoryImpl();
  const me = await repo.getMe(authUser.data);
  if (!me.ok) {
    return NextResponse.json({ ok: false, error: me.error }, { status: me.status });
  }
  return NextResponse.json({ ok: true, data: me.data }, { status: 200 });
}

export async function DELETE(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const repo = new UserRepositoryImpl();
  const deleted = await repo.deleteMe(authUser.data);
  if (!deleted.ok) {
    return NextResponse.json({ ok: false, error: deleted.error }, { status: deleted.status });
  }
  return NextResponse.json({ ok: true, data: deleted.data }, { status: 200 });
}

export async function POST(req: NextRequest) {
  const authUser = await authenticateRequest(req);
  if (!authUser.ok) {
    return NextResponse.json({ ok: false, error: authUser.error }, { status: authUser.status });
  }

  const repo = new UserRepositoryImpl();
  const revoked = await repo.revokeMe(authUser.data);
  if (!revoked.ok) {
    return NextResponse.json({ ok: false, error: revoked.error }, { status: revoked.status });
  }
  return NextResponse.json({ ok: true, data: revoked.data }, { status: 200 });
}
