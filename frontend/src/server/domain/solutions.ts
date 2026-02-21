import {
  SolutionComment,
  SolutionDetail,
  SolutionLikeStatus,
  SolutionListItem,
  SolutionListSortBy,
  SolutionVotesCount,
} from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";
import { Resp } from "../response";

export interface SolutionRepository {
  create(solution: Solution, token: string): Promise<Resp<SolutionResponse>>;
  getBySolutionId(solutionId: string): Promise<Resp<SolutionDetail>>;
  getByProblemId(problemId: string, sortBy?: SolutionListSortBy): Promise<Resp<SolutionListItem[]>>;
  getCommentsBySolutionId(solutionId: string): Promise<Resp<SolutionComment[]>>;
  createComment(solutionId: string, bodyMd: string, token: string): Promise<Resp<SolutionComment>>;
  vote(solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>>;
  unvote(solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>>;
  getVotesCount(solutionId: string): Promise<Resp<SolutionVotesCount>>;
  getMyVoteStatus(solutionId: string, token: string): Promise<Resp<SolutionLikeStatus>>;
}
