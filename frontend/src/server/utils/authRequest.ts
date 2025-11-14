import { NextRequest } from "next/server";
import { auth, firebaseAdmin } from "@/shared/firebase/backend";
import { Resp } from "../response";

export async function authenticateRequest(request: NextRequest): Promise<Resp<string>> {
  const authHeader = request.headers.get("Authorization");

  if (!authHeader || !authHeader.startsWith("Bearer ")) {
    return {
      status: 401,
      error: "認証できません: トークンが不正です",
      ok: false,
    };
  }
  const token = authHeader.split(" ")[1];

  if (!firebaseAdmin) {
    return {
      status: 500,
      error: "Internal Server Error",
      ok: false,
    };
  }

  try {
    const _decodedToken = await auth.verifyIdToken(token);
    return {
      data: token,
      status: 200,
      ok: true,
    };
  } catch {
    return {
      status: 401,
      error: "Invalid Token",
      ok: false,
    };
  }
}
