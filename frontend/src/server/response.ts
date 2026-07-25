export type Resp<T> = { ok: true; data: T; status: 200 } | { ok: false; error: string; errorCode?: string; status: number };
