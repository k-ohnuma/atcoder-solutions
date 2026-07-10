import { z } from "zod";

export const updateSolutionFormSchema = z.object({
  title: z.string().trim().min(1, "タイトルを入力してください").max(120, "タイトルは120文字以内にしてください"),
  bodyMd: z.string().trim().min(1, "本文を入力してください").max(20000, "本文は20000文字以内にしてください"),
  submitUrl: z
    .string()
    .trim()
    .max(500, "提出URLは500文字以内にしてください")
    .refine((v) => v.length === 0 || /^https?:\/\//.test(v), "URLは http(s) から始めてください"),
  tags: z
    .array(z.string().trim().min(1, "タグは空にできません").max(24, "タグは24文字以内にしてください"))
    .max(6, "タグは最大6個です"),
});

export type UpdateSolutionFormInput = z.infer<typeof updateSolutionFormSchema>;
