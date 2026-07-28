export type Resp<T> = { ok: true; data: T; status: 200 } | { ok: false; error: string; errorCode?: string; status: number };

type JsonValue = string;
type QueryParams = Record<string, string>;
type RequestHeaders = Record<string, string>;

function bearerHeader(token: string) {
  return `Bearer ${token}`;
}

async function request<T = unknown>({
  method,
  path,
  token,
  body,
  params,
  headers,
}: {
  method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
  path: string;
  token?: string;
  body?: JsonValue;
  params?: QueryParams;
  headers?: RequestHeaders;
}): Promise<Resp<T>> {
  const url = new URL(path, window.location.origin);

  if (params) {
    for (const [key, value] of Object.entries(params)) {
      url.searchParams.append(key, value);
    }
  }

  const reqHeaders: Record<string, string> = {
    Accept: "application/json",
    ...headers,
  };

  if (token) {
    reqHeaders.Authorization = bearerHeader(token);
  }
  const init: RequestInit = {
    method,
    headers: reqHeaders,
  };
  if (body !== undefined) {
    reqHeaders["Content-Type"] = "application/json";
    init.body = body;
  }

  try {
    const res = await fetch(url, init);
    const json = (await res.json()) as Resp<T>;
    return {
      ...json,
      status: typeof json.status === "number" ? json.status : res.status,
    } as Resp<T>;
  } catch (e) {
    console.error(`failed to send a request ${url}, method: ${method}, ${e}`.trim());
    return {
      ok: false,
      status: 500,
      error: "internal server error",
    };
  }
}

export const httpClient = {
  get: <T>(path: string, params?: QueryParams): Promise<Resp<T>> => request({ method: "GET", path, params }),
  getWithToken: <T>(path: string, token: string, params?: QueryParams): Promise<Resp<T>> =>
    request({ method: "GET", path, token, params }),
  post: <T>(path: string, body?: JsonValue, headers?: RequestHeaders): Promise<Resp<T>> =>
    request({ method: "POST", path, body, headers }),
  postWithToken: <T>(path: string, token: string, body?: JsonValue, headers?: RequestHeaders): Promise<Resp<T>> =>
    request({ method: "POST", path, token, body, headers }),
  putWithToken: <T>(path: string, token: string, body?: JsonValue): Promise<Resp<T>> =>
    request({ method: "PUT", path, token, body }),
  patchWithToken: <T>(path: string, token: string, body: JsonValue): Promise<Resp<T>> =>
    request({ method: "PATCH", path, token, body }),
  deleteWithToken: <T>(path: string, token: string, params?: QueryParams): Promise<Resp<T>> =>
    request({ method: "DELETE", path, token, params }),
};
