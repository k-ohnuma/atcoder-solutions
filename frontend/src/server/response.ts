export type Resp<T> = { ok: true; data: T; status: 200 } | { ok: false; error: string; status: number };
