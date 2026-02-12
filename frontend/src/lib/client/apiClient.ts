import { ContestGroupCollection } from "@/server/domain/problems";
import { Problem } from "@/shared/model/problem";
import { SolutionLikeStatus, SolutionVotesCount } from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";

export type Resp<T> = { ok: true; data: T; status: 200 } | { ok: false; error: string; status: number };

type JsonValue = string;

type QueryParams = Record<string, string>;

export class ApiClient {
  constructor() {
    return;
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
    method: "GET" | "POST" | "DELETE";
    path: string;
    token?: string;
    body?: JsonValue;
    params?: Record<string, string>;
  }): Promise<Resp<T>> {
    const url = new URL(path, window.location.origin);

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
      init.body = body;
    }

    try {
      const res = await fetch(url, init);
      return await res.json();
    } catch (e) {
      console.error(`failed to send a request ${url}, method: ${method}, ${e}`.trim());
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

  private async getWithToken<T>(path: string, token: string, params?: QueryParams): Promise<Resp<T>> {
    return this.request({ method: "GET", path, token, params });
  }

  // private async _post<T>(path: string, body: JsonValue): Promise<Resp<T>> {
  //   return this.request({ path, method: "POST", body });
  // }
  //
  private async postWithToken<T>(path: string, token: string, body: JsonValue): Promise<Resp<T>> {
    return this.request({ path, method: "POST", body, token });
  }

  private async deleteWithToken<T>(path: string, token: string, params?: QueryParams): Promise<Resp<T>> {
    return this.request({ path, method: "DELETE", token, params });
  }

  // contests
  getContestsByContestSeries = async <T>(contestSeries: string): Promise<T> => {
    const path = "api/contests";
    const resp = await this.get<T>(path, { series: contestSeries });
    if (resp.ok) return resp.data;
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return [] as T;
  };
  // problems
  getProblemsByContest = async <T>(contest: string): Promise<T> => {
    const path = "api/problems";
    const resp = await this.get<T>(path, { contest });
    if (resp.ok) return resp.data;
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return [] as T;
  };
  getContestGroupByContestSeries = async (contestSeries: string): Promise<ContestGroupCollection> => {
    const path = "api/problems/contest-group";
    const resp = await this.get<Record<string, Problem[]>>(path, {
      series: contestSeries,
    });
    if (resp.ok) {
      const map: Map<string, Problem[]> = new Map<string, Problem[]>(
        Object.entries(resp.data).map(([contestId, problems]) => [contestId, problems]),
      );
      return map;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return new Map();
  };
  createSolution = async (solution: Solution, token: string): Promise<SolutionResponse> => {
    const path = "api/solutions";
    const resp = await this.postWithToken<SolutionResponse>(path, token, JSON.stringify(solution));
    if (resp.ok) return resp.data;
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return { solutionId: "" };
  };

  getSolutionVotesCount = async (solutionId: string): Promise<number> => {
    const path = "api/solutions/votes";
    const resp = await this.get<SolutionVotesCount>(path, { solutionId });
    if (resp.ok) {
      return resp.data.votesCount;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return 0;
  };

  getMySolutionLikeStatus = async (solutionId: string, token: string): Promise<boolean> => {
    const path = "api/solutions/votes/me";
    const resp = await this.getWithToken<SolutionLikeStatus>(path, token, { solutionId });
    if (resp.ok) {
      return resp.data.liked;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return false;
  };

  voteSolution = async (solutionId: string, token: string): Promise<boolean> => {
    const path = "api/solutions/votes";
    const resp = await this.postWithToken<SolutionLikeStatus>(path, token, JSON.stringify({ solutionId }));
    if (resp.ok) {
      return resp.data.liked;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return false;
  };

  unvoteSolution = async (solutionId: string, token: string): Promise<boolean> => {
    const path = "api/solutions/votes";
    const resp = await this.deleteWithToken<SolutionLikeStatus>(path, token, { solutionId });
    if (resp.ok) {
      return resp.data.liked;
    }
    console.log(`error: ${resp.error}, status: ${resp.status}`);
    return true;
  };
}
