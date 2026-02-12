export interface SolutionDetail {
  id: string;
  title: string;
  problemId: string;
  problemTitle: string;
  userId: string;
  userName: string;
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
  createdAt: string;
  updatedAt: string;
}
