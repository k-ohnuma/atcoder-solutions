export type DifficultyBand = "gray" | "brown" | "green" | "cyan" | "blue" | "yellow" | "orange" | "red";

export interface Problem {
  id: string;
  contestCode: string;
  problemIndex: string;
  title: string;
  difficulty?: number | null;
  difficultyBand?: DifficultyBand | null;
}
