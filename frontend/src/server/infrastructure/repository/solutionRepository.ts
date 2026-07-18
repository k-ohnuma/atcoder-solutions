import "server-only";

import { SolutionRepository } from "@/server/domain/solutions";
import { Resp } from "@/server/response";
import { BackendApiClient } from "@/server/utils/apiClient";
import { serverConfig } from "@/shared/config/backend";
import {
  SolutionComment,
  SolutionDetail,
  SolutionLikeStatus,
  SolutionListItem,
  SolutionListSortBy,
  SolutionVotesCount,
  UserSolutionListItem,
} from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";

export class SolutionRepositoryImpl implements SolutionRepository {
  private client: BackendApiClient;
  private static readonly NO_STORE = { kind: "no-store" } as const;
  constructor() {
    const baseEndpoint = serverConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private createSolutionPath = () => {
    return "solutions";
  };
  private solutionPath = (solutionId: string) => {
    return `solutions/${encodeURIComponent(solutionId)}`;
  };
  private getSolutionsByProblemIdPath = () => {
    return "problems";
  };
  private getSolutionsByUserNamePath = () => {
    return "users";
  };
  private getLatestSolutionsPath = () => {
    return "solutions";
  };
  private votesPath = (solutionId: string) => {
    return `solutions/${encodeURIComponent(solutionId)}/votes`;
  };
  private myVotesPath = (solutionId: string) => {
    return `solutions/${encodeURIComponent(solutionId)}/votes/me`;
  };
  private commentsPath = (solutionId: string) => {
    return `solutions/${encodeURIComponent(solutionId)}/comments`;
  };
  private commentPath = (commentId: string) => {
    return `comments/${encodeURIComponent(commentId)}`;
  };

  create = async (solution: Solution, token: string): Promise<Resp<SolutionResponse>> => {
    const path = this.createSolutionPath();
    return await this.client.postWithToken<SolutionResponse>(path, token, JSON.stringify(solution));
  };

  update = async (
    solutionId: string,
    solution: Pick<Solution, "title" | "bodyMd" | "submitUrl" | "tags">,
    token: string,
  ): Promise<Resp<SolutionResponse>> => {
    const path = this.solutionPath(solutionId);
    return await this.client.patchWithToken<SolutionResponse>(path, token, JSON.stringify(solution));
  };

  delete = async (solutionId: string, token: string): Promise<Resp<{ solutionId: string }>> => {
    const path = this.solutionPath(solutionId);
    return await this.client.deleteWithToken<{ solutionId: string }>(path, token);
  };

  getBySolutionId = async (solutionId: string): Promise<Resp<SolutionDetail>> => {
    const path = this.solutionPath(solutionId);
    return await this.client.get<SolutionDetail>(path, undefined, SolutionRepositoryImpl.NO_STORE);
  };

  getLatest = async (limit?: number): Promise<Resp<SolutionListItem[]>> => {
    const path = this.getLatestSolutionsPath();
    const params: Record<string, string> = { sortBy: "latest" };
    if (limit !== undefined) {
      params.limit = limit.toString();
    }
    return await this.client.get<SolutionListItem[]>(path, params, SolutionRepositoryImpl.NO_STORE);
  };

  getByProblemId = async (
    problemId: string,
    sortBy?: SolutionListSortBy,
    limit?: number,
  ): Promise<Resp<SolutionListItem[]>> => {
    const path = `${this.getSolutionsByProblemIdPath()}/${encodeURIComponent(problemId)}/solutions`;
    const params: Record<string, string> = {};
    if (sortBy) {
      params.sortBy = sortBy;
    }
    if (limit !== undefined) {
      params.limit = limit.toString();
    }
    return await this.client.get<SolutionListItem[]>(path, params, SolutionRepositoryImpl.NO_STORE);
  };

  getByUserName = async (userName: string, sortBy?: SolutionListSortBy): Promise<Resp<UserSolutionListItem[]>> => {
    const path = `${this.getSolutionsByUserNamePath()}/${encodeURIComponent(userName)}/solutions`;
    const params: Record<string, string> = {};
    if (sortBy) {
      params.sortBy = sortBy;
    }
    return await this.client.get<UserSolutionListItem[]>(path, params, SolutionRepositoryImpl.NO_STORE);
  };

  getCommentsBySolutionId = async (solutionId: string): Promise<Resp<SolutionComment[]>> => {
    const path = this.commentsPath(solutionId);
    return await this.client.get<SolutionComment[]>(path, undefined, SolutionRepositoryImpl.NO_STORE);
  };

  createComment = async (solutionId: string, bodyMd: string, token: string): Promise<Resp<SolutionComment>> => {
    const path = this.commentsPath(solutionId);
    return await this.client.postWithToken<SolutionComment>(path, token, JSON.stringify({ bodyMd }));
  };

  updateComment = async (commentId: string, bodyMd: string, token: string): Promise<Resp<SolutionComment>> => {
    const path = this.commentPath(commentId);
    return await this.client.patchWithToken<SolutionComment>(path, token, JSON.stringify({ bodyMd }));
  };

  deleteComment = async (commentId: string, token: string): Promise<Resp<{ commentId: string }>> => {
    const path = this.commentPath(commentId);
    return await this.client.deleteWithToken<{ commentId: string }>(path, token);
  };

  vote = async (solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>> => {
    const path = this.myVotesPath(solutionId);
    return await this.client.putWithToken<SolutionLikeStatus>(path, token);
  };

  unvote = async (solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>> => {
    const path = this.myVotesPath(solutionId);
    return await this.client.deleteWithToken<SolutionLikeStatus>(path, token);
  };

  getVotesCount = async (solutionId: string): Promise<Resp<SolutionVotesCount>> => {
    const path = this.votesPath(solutionId);
    return await this.client.get<SolutionVotesCount>(path, undefined, SolutionRepositoryImpl.NO_STORE);
  };

  getMyVoteStatus = async (solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>> => {
    const path = this.myVotesPath(solutionId);
    return await this.client.getWithToken<SolutionLikeStatus>(path, token, undefined, SolutionRepositoryImpl.NO_STORE);
  };
}
