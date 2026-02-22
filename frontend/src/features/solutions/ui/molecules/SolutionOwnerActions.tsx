"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ApiClient } from "@/lib/client/apiClient";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { MarkdownRenderer } from "../atoms";
import { MarkdownEditor } from "./MarkdownEditor";
import { TagsInputField } from "./TagInput";

const apiClient = new ApiClient();

const updateSolutionFormSchema = z.object({
  title: z.string().trim().min(1, "タイトルを入力してください").max(120, "タイトルは120文字以内にしてください"),
  bodyMd: z.string().trim().min(1, "本文を入力してください").max(20000, "本文は20000文字以内にしてください"),
  submitUrl: z
    .string()
    .trim()
    .max(500, "提出URLは500文字以内にしてください")
    .refine((v) => v.length === 0 || /^https?:\/\//.test(v), "URLは http(s) から始めてください"),
  tags: z.array(z.string().trim().min(1, "タグは空にできません").max(24, "タグは24文字以内にしてください")).max(6, "タグは最大6個です"),
});

type UpdateSolutionFormInput = z.infer<typeof updateSolutionFormSchema>;

type SolutionOwnerActionsProps = {
  solutionId: string;
  problemId: string;
  ownerUserId: string;
  title: string;
  bodyMd: string;
  submitUrl: string;
  tags: string[];
};

export function SolutionOwnerActions({
  solutionId,
  problemId,
  ownerUserId,
  title,
  bodyMd,
  submitUrl,
  tags,
}: SolutionOwnerActionsProps) {
  const router = useRouter();
  const auth = getFirebaseAuth();
  const [currentUserId, setCurrentUserId] = useState<string | null>(null);
  const [editing, setEditing] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const form = useForm<UpdateSolutionFormInput>({
    resolver: zodResolver(updateSolutionFormSchema),
    defaultValues: {
      title,
      bodyMd,
      submitUrl,
      tags,
    },
    mode: "onSubmit",
  });

  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
    reset,
  } = form;

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      setCurrentUserId(user?.uid ?? null);
    });
    return () => unsub();
  }, [auth]);

  useEffect(() => {
    reset({
      title,
      bodyMd,
      submitUrl,
      tags,
    });
  }, [bodyMd, reset, submitUrl, tags, title]);

  const isOwner = useMemo(() => currentUserId !== null && currentUserId === ownerUserId, [currentUserId, ownerUserId]);

  if (!isOwner) {
    return null;
  }

  const save = async (values: UpdateSolutionFormInput) => {
    if (isSubmitting) {
      return;
    }
    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return;
    }
    setIsSubmitting(true);
    setError(null);
    try {
      const normalizedTags = values.tags.map((v) => v.trim()).filter((v) => v.length > 0);
      const ok = await apiClient.updateSolution(
        solutionId,
        values.title,
        values.bodyMd,
        values.submitUrl,
        normalizedTags,
        token,
      );
      if (!ok) {
        setError("解説の更新に失敗しました。");
        return;
      }
      setEditing(false);
      router.refresh();
    } finally {
      setIsSubmitting(false);
    }
  };

  const remove = async () => {
    if (isSubmitting) {
      return;
    }
    if (!confirm("この解説を削除しますか？")) {
      return;
    }
    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return;
    }
    setIsSubmitting(true);
    setError(null);
    try {
      const ok = await apiClient.deleteSolution(solutionId, token);
      if (!ok) {
        setError("解説の削除に失敗しました。");
        return;
      }
      router.push(`/problems/${problemId}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <section className="rounded-xl border bg-card p-3">
      <div className="flex flex-wrap items-center gap-2">
        <Button type="button" size="sm" variant="outline" onClick={() => setEditing((v) => !v)} disabled={isSubmitting}>
          {editing ? "編集を閉じる" : "解説を編集"}
        </Button>
        <Button type="button" size="sm" variant="destructive" onClick={remove} disabled={isSubmitting}>
          解説を削除
        </Button>
      </div>

      {editing && (
        <form className="mt-3 space-y-2" onSubmit={handleSubmit(save)}>
          <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
            <p className="pt-2 text-sm font-medium text-muted-foreground">タイトル</p>
            <div>
              <input className="w-full rounded-md border px-3 py-2 text-sm" {...register("title")} placeholder="タイトル" />
              {errors.title && <p className="mt-1 text-xs text-destructive">{errors.title.message}</p>}
            </div>
          </div>
          <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
            <p className="pt-2 text-sm font-medium text-muted-foreground">提出URL</p>
            <div>
              <input
                className="w-full rounded-md border px-3 py-2 text-sm"
                {...register("submitUrl")}
                placeholder="提出URL（任意）"
              />
              {errors.submitUrl && <p className="mt-1 text-xs text-destructive">{errors.submitUrl.message}</p>}
            </div>
          </div>
          <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
            <p className="pt-2 text-sm font-medium text-muted-foreground">タグ</p>
            <div>
              <Controller
                control={control}
                name="tags"
                render={({ field }) => (
                  <TagsInputField
                    label="タグ"
                    values={field.value}
                    onChangeAction={field.onChange}
                    className="w-full"
                    hideLabel
                  />
                )}
              />
              {errors.tags && <p className="mt-1 text-xs text-destructive">{errors.tags.message}</p>}
            </div>
          </div>
          <div className="grid items-start gap-2 md:grid-cols-[120px_1fr]">
            <p className="pt-2 text-sm font-medium text-muted-foreground">本文</p>
            <div>
              <Controller
                control={control}
                name="bodyMd"
                render={({ field }) => (
                  <Tabs defaultValue="write" className="gap-2">
                    <TabsList>
                      <TabsTrigger value="write">入力</TabsTrigger>
                      <TabsTrigger value="preview">プレビュー</TabsTrigger>
                    </TabsList>
                    <TabsContent value="write">
                      <div className="h-56">
                        <MarkdownEditor value={field.value} onChangeAction={field.onChange} />
                      </div>
                    </TabsContent>
                    <TabsContent value="preview">
                      <div className="min-h-56 rounded-md border bg-muted/30 p-3">
                        <MarkdownRenderer value={field.value || "（プレビューはここに表示されます）"} />
                      </div>
                    </TabsContent>
                  </Tabs>
                )}
              />
              {errors.bodyMd && <p className="mt-1 text-xs text-destructive">{errors.bodyMd.message}</p>}
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Button type="submit" size="sm" disabled={isSubmitting}>
              {isSubmitting ? "保存中..." : "更新を保存"}
            </Button>
          </div>
        </form>
      )}
      {error && <p className="mt-2 text-sm text-destructive">{error}</p>}
    </section>
  );
}
