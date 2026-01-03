import { z } from "zod";

export const createSolutionSchema = z.object({
  problemId: z.string().min(1, "問題を選択してください"),
  title: z.string().min(1, "タイトルは必須です").max(100, "タイトルは100文字以内にしてください"),
  tags: z.array(z.string().min(1)).max(6, "タグは最大6個です"),
  submitUrl: z
    .string()
    .trim()
    .refine((v) => !v || /^https?:\/\//.test(v), "URLは http(s) から始めてください"),
  bodyMd: z.string().min(1, "本文は必須です"),
});

export type CreateSolutionInput = z.infer<typeof createSolutionSchema>;

export interface Solution {
  problemId: string;
  title: string;
  bodyMd: string;
  submitUrl: string;
  tags: string[];
}

export interface SolutionResponse {
  solutionId: string;
}
