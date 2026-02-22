import { z } from "zod";

export const createSolutionSchema = z.object({
  problemId: z.string().trim().min(1, "問題を選択してください"),
  title: z.string().trim().min(1, "タイトルは必須です").max(120, "タイトルは120文字以内にしてください"),
  tags: z
    .array(z.string().trim().min(1, "タグは空にできません").max(24, "タグは24文字以内にしてください"))
    .max(6, "タグは最大6個です"),
  submitUrl: z
    .string()
    .trim()
    .max(500, "提出URLは500文字以内にしてください")
    .refine((v) => v.length === 0 || /^https?:\/\//.test(v), "URLは http(s) から始めてください"),
  bodyMd: z.string().trim().min(1, "本文は必須です").max(20000, "本文は20000文字以内にしてください"),
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
