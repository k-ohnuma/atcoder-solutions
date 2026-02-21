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
} from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";

export class SolutionRepositoryImpl implements SolutionRepository {
  private client: BackendApiClient;
  constructor() {
    const baseEndpoint = serverConfig.appConfig.apiBaseEndpoint;
    this.client = new BackendApiClient(baseEndpoint);
  }

  private createSolutionPath = () => {
    return "solutions";
  };
  private getSolutionsByProblemIdPath = () => {
    return "solutions/problems";
  };
  private votesPath = () => {
    return "solutions/votes";
  };
  private myVotesPath = () => {
    return "solutions/votes/me";
  };
  private commentsPath = () => {
    return "solutions/comments";
  };

  create = async (solution: Solution, token: string): Promise<Resp<SolutionResponse>> => {
    const path = this.createSolutionPath();
    return await this.client.postWithToken<SolutionResponse>(path, token, JSON.stringify(solution));
  };

  getBySolutionId = async (solutionId: string): Promise<Resp<SolutionDetail>> => {
    const path = this.createSolutionPath();
    const params = { solutionId };
    return await this.client.get<SolutionDetail>(path, params);
  };

  getByProblemId = async (problemId: string, sortBy?: SolutionListSortBy): Promise<Resp<SolutionListItem[]>> => {
    const path = this.getSolutionsByProblemIdPath();
    const params: Record<string, string> = { problemId };
    if (sortBy) {
      params.sortBy = sortBy;
    }
    return await this.client.get<SolutionListItem[]>(path, params);
  };

  getCommentsBySolutionId = async (solutionId: string): Promise<Resp<SolutionComment[]>> => {
    const path = this.commentsPath();
    const params = { solutionId };
    return await this.client.get<SolutionComment[]>(path, params);
  };

  createComment = async (solutionId: string, bodyMd: string, token: string): Promise<Resp<SolutionComment>> => {
    const path = this.commentsPath();
    return await this.client.postWithToken<SolutionComment>(path, token, JSON.stringify({ solutionId, bodyMd }));
  };

  vote = async (solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>> => {
    const path = this.votesPath();
    return await this.client.postWithToken<SolutionLikeStatus>(path, token, JSON.stringify({ solutionId }));
  };

  unvote = async (solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>> => {
    const path = this.votesPath();
    const params = { solutionId };
    return await this.client.deleteWithToken<SolutionLikeStatus>(path, token, params);
  };

  getVotesCount = async (solutionId: string): Promise<Resp<SolutionVotesCount>> => {
    const path = this.votesPath();
    const params = { solutionId };
    return await this.client.get<SolutionVotesCount>(path, params);
  };

  getMyVoteStatus = async (solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>> => {
    const path = this.myVotesPath();
    const params = { solutionId };
    return await this.client.getWithToken<SolutionLikeStatus>(path, token, params);
  };
}
