import { SolutionListItem, SolutionListSortBy, UserSolutionListItem } from "@/shared/model/solution";

export type SolutionSummaryListItem = {
  id: string;
  title: string;
  votesCount: number;
  createdAt: string;
  problemId?: string;
  problemTitle?: string;
  userName?: string;
};

export function normalizeSolutionListSort(sortBy?: string): SolutionListSortBy {
  return sortBy === "votes" ? "votes" : "latest";
}

export function toSolutionSummaryListItem(solution: SolutionListItem): SolutionSummaryListItem {
  return {
    id: solution.id,
    title: solution.title,
    votesCount: solution.votesCount,
    createdAt: solution.createdAt,
    problemId: solution.problemId,
    userName: solution.userName,
  };
}

export function toUserSolutionSummaryListItem(solution: UserSolutionListItem): SolutionSummaryListItem {
  return {
    id: solution.id,
    title: solution.title,
    votesCount: solution.votesCount,
    createdAt: solution.createdAt,
    problemId: solution.problemId,
    problemTitle: solution.problemTitle,
  };
}
