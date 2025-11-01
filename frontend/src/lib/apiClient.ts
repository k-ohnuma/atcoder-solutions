import { ContestGroupCollection } from "@/server/domain/problems";
import { Problem } from "@/shared/model/problem";

export type Resp<T> =
  | { ok: true; data: T; status: 200 }
  | { ok: false; error: string; status: number };

type JsonValue =
  | string
  | number
  | boolean
  | null
  | JsonValue[]
  | { [key: string]: JsonValue };

type QueryParams = Record<string, string>;

export class ApiClient {
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
      Accept: "application/json",
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
  private async get<T>(path: string, params?: QueryParams): Promise<Resp<T>> {
    return this.request({ method: "GET", path, params });
  }

  private async getWithToken<T>(path: string, token: string): Promise<Resp<T>> {
    return this.request({ method: "GET", path, token });
  }

  private async post<T>(path: string, body: JsonValue): Promise<Resp<T>> {
    return this.request({ path, method: "POST", body });
  }

  private async postWithToken<T>(
    path: string,
    token: string,
    body: JsonValue,
  ): Promise<Resp<T>> {
    return this.request({ path, method: "POST", body, token });
  }

  // problems
  getProblemsByContestSeries = async <T>(contestSeries: string): Promise<T> => {
    const path = "problems";
    const resp = await this.get<T>(path, { series: contestSeries });
    if (resp.ok) return resp.data;
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return [] as T;
  };
  getContestGroupByContestSeries = async (
    contestSeries: string,
  ): Promise<ContestGroupCollection> => {
    const path = "problems/contest-group";
    const resp = await this.get<Record<string, Problem[]>>(path, {
      series: contestSeries,
    });
    if (resp.ok) {
      const map: Map<string, Problem[]> = new Map<string, Problem[]>(
        Object.entries(resp.data).map(([contestId, problems]) => [
          contestId,
          problems,
        ]),
      );
      return map;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return new Map();
  };
}
