import "server-only";
import { Resp } from "../response";

type JsonValue =
  | string
  | number
  | boolean
  | null
  | JsonValue[]
  | { [key: string]: JsonValue };

type QueryParams = Record<string, string>;

export class BackendApiClient {
  private baseEndpoint: string;
  constructor(baseEndpoint: string) {
    this.baseEndpoint = baseEndpoint;
  }

  private bearerHeader(token: string) {
    return `Bearer ${token}`;
  }

  private async request<T = unknown>({
    method,
    path,
    token,
    body,
    params,
  }: {
    method: "GET" | "POST";
    path: string;
    token?: string;
    body?: JsonValue;
    params?: Record<string, string>;
  }): Promise<Resp<T>> {
    const url = new URL(path.replace(/^\//, ""), this.baseEndpoint);

    if (params) {
      for (const [key, value] of Object.entries(params)) {
        url.searchParams.append(key, value);
      }
    }

    const reqHeaders: Record<string, string> = {
      Accept: "application/json, text/plain;q=0.8, */*;q=0.5",
    };

    if (token) reqHeaders.Authorization = this.bearerHeader(token);
    const hasBody = body !== undefined;
    const init: RequestInit = {
      method,
      headers: reqHeaders,
    };
    if (hasBody) {
      if (!reqHeaders["Content-Type"]) {
        reqHeaders["Content-Type"] = "application/json";
      }
      init.body = JSON.stringify(body);
    }

    try {
      const res = await fetch(url, init);
      return await res.json();
    } catch (e) {
      console.error(
        `failed to send a request ${url}, method: ${method}, ${e}`.trim(),
      );
      return {
        ok: false,
        status: 500,
        error: "internal server error",
      };
    }
  }

  async get<T>(path: string, params?: QueryParams): Promise<Resp<T>> {
    return this.request({ method: "GET", path, params });
  }

  async getWithToken<T>(path: string, token: string): Promise<Resp<T>> {
    return this.request({ method: "GET", path, token });
  }

  async post<T>(path: string, body: JsonValue): Promise<Resp<T>> {
    return this.request({ path, method: "POST", body });
  }

  async postWithToken<T>(
    path: string,
    token: string,
    body: JsonValue,
  ): Promise<Resp<T>> {
    return this.request({ path, method: "POST", body, token });
  }
}
