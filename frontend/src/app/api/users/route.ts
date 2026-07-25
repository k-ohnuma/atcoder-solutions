import { NextRequest, NextResponse } from "next/server";
import { UserRepositoryImpl } from "@/server/infrastructure/repository/userRepository";
import { createUserRequest } from "@/server/interface/user/create";

export async function POST(req: NextRequest) {
  const json = await req.json();
  const parsed = createUserRequest.safeParse(json);
  if (!parsed.success) {
    const message = parsed.error.issues[0]?.message ?? "invalid format";
    return NextResponse.json({ ok: false, error: message }, { status: 400 });
  }

  const repo = new UserRepositoryImpl();
  const created = await repo.create(parsed.data);
  if (!created.ok) {
    return NextResponse.json({ ok: false, error: created.error, errorCode: created.errorCode }, { status: created.status });
  }
  return NextResponse.json({ ok: true, data: created.data }, { status: 200 });
}
