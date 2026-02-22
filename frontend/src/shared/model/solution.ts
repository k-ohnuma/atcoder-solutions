export interface SolutionDetail {
  id: string;
  title: string;
  problemId: string;
  problemTitle: string;
  userId: string;
  userName: string;
  tags: string[];
  bodyMd: string;
  submitUrl: string;
  createdAt: string;
  updatedAt: string;
}

export interface SolutionListItem {
  id: string;
  title: string;
  problemId: string;
  userId: string;
  userName: string;
  votesCount: number;
  createdAt: string;
  updatedAt: string;
}

export interface UserSolutionListItem {
  id: string;
  title: string;
  problemId: string;
  problemTitle: string;
  userId: string;
  userName: string;
  votesCount: number;
  createdAt: string;
  updatedAt: string;
}

export interface SolutionVotesCount {
  solutionId: string;
  votesCount: number;
}

export interface SolutionLikeStatus {
  solutionId: string;
  liked: boolean;
}

export interface SolutionComment {
  id: string;
  userId: string;
  userName: string;
  solutionId: string;
  bodyMd: string;
  createdAt: string;
  updatedAt: string;
}

export type SolutionListSortBy = "latest" | "votes";
