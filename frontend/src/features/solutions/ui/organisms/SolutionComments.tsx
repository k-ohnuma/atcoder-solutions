"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ApiClient } from "@/lib/client/apiClient";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { SolutionComment } from "@/shared/model/solution";
import { MarkdownRenderer } from "../atoms";
import { MarkdownEditor } from "../molecules";

const apiClient = new ApiClient();

const commentEditSchema = z.object({
  bodyMd: z.string().trim().min(1, "コメントを入力してください").max(2000, "コメントは2000文字以内で入力してください"),
});

type CommentEditInput = z.infer<typeof commentEditSchema>;

const dateTimeFormatter = new Intl.DateTimeFormat("ja-JP", {
  timeZone: "Asia/Tokyo",
  year: "numeric",
  month: "2-digit",
  day: "2-digit",
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  hour12: false,
});

type SolutionCommentsProps = {
  solutionId: string;
  initialComments: SolutionComment[];
};

export function SolutionComments({ solutionId, initialComments }: SolutionCommentsProps) {
  const router = useRouter();
  const auth = getFirebaseAuth();
  const [comments, setComments] = useState(initialComments);
  const [bodyMd, setBodyMd] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentUserId, setCurrentUserId] = useState<string | null>(null);
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null);
  const editForm = useForm<CommentEditInput>({
    resolver: zodResolver(commentEditSchema),
    defaultValues: {
      bodyMd: "",
    },
    mode: "onSubmit",
  });
  const {
    handleSubmit: handleEditSubmit,
    watch: watchEdit,
    setValue: setEditValue,
    reset: resetEdit,
    formState: { errors: editErrors },
  } = editForm;
  const editingBodyMd = watchEdit("bodyMd");

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      setCurrentUserId(user?.uid ?? null);
    });
    return () => unsub();
  }, [auth]);

  const onSubmit = async () => {
    if (isSubmitting) {
      return;
    }
    const normalized = bodyMd.trim();
    if (!normalized) {
      setError("コメントを入力してください");
      return;
    }
    if (normalized.length > 2000) {
      setError("コメントは2000文字以内で入力してください");
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
      const created = await apiClient.createComment(solutionId, normalized, token);
      if (!created) {
        setError("コメントの投稿に失敗しました");
        return;
      }

      setComments((prev) => [...prev, created]);
      setBodyMd("");
    } finally {
      setIsSubmitting(false);
    }
  };

  const onStartEditComment = (comment: SolutionComment) => {
    setEditingCommentId(comment.id);
    resetEdit({ bodyMd: comment.bodyMd });
    setError(null);
  };

  const onUpdateComment = async (commentId: string, values: CommentEditInput) => {
    if (isSubmitting) {
      return;
    }
    const normalized = values.bodyMd.trim();

    const token = await getFirebaseIdToken();
    if (!token) {
      router.push("/signin");
      return;
    }

    setIsSubmitting(true);
    setError(null);
    try {
      const updated = await apiClient.updateComment(commentId, normalized, token);
      if (!updated) {
        setError("コメントの更新に失敗しました");
        return;
      }
      setComments((prev) => prev.map((comment) => (comment.id === commentId ? updated : comment)));
      setEditingCommentId(null);
      resetEdit({ bodyMd: "" });
    } finally {
      setIsSubmitting(false);
    }
  };

  const onDeleteComment = async (commentId: string) => {
    if (isSubmitting) {
      return;
    }
    if (!confirm("このコメントを削除しますか？")) {
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
      const ok = await apiClient.deleteComment(commentId, token);
      if (!ok) {
        setError("コメントの削除に失敗しました");
        return;
      }
      setComments((prev) => prev.filter((comment) => comment.id !== commentId));
      if (editingCommentId === commentId) {
        setEditingCommentId(null);
        resetEdit({ bodyMd: "" });
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <section className="space-y-4">
      <h2 className="text-xl font-semibold">コメント</h2>
      <div className="space-y-3">
        {comments.length === 0 ? (
          <p className="text-sm text-muted-foreground">まだコメントはありません。</p>
        ) : (
          comments.map((comment) => (
            <article key={comment.id} className="rounded-xl border p-4">
              <div className="mb-3 flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
                <span>{comment.userName}</span>
                <span>{dateTimeFormatter.format(new Date(comment.createdAt))}</span>
                {currentUserId === comment.userId && (
                  <span className="ml-auto flex items-center gap-2">
                    <button type="button" className="text-xs hover:underline" onClick={() => onStartEditComment(comment)}>
                      編集
                    </button>
                    <button
                      type="button"
                      className="text-xs text-destructive hover:underline"
                      onClick={() => onDeleteComment(comment.id)}
                    >
                      削除
                    </button>
                  </span>
                )}
              </div>
              {editingCommentId === comment.id ? (
                <div className="space-y-2">
                  <Tabs defaultValue="write" className="gap-2">
                    <TabsList>
                      <TabsTrigger value="write">入力</TabsTrigger>
                      <TabsTrigger value="preview">プレビュー</TabsTrigger>
                    </TabsList>
                    <TabsContent value="write">
                      <div className="h-40">
                        <MarkdownEditor value={editingBodyMd} onChangeAction={(v) => setEditValue("bodyMd", v)} />
                      </div>
                    </TabsContent>
                    <TabsContent value="preview">
                      <div className="min-h-40 rounded-md border bg-muted/30 p-2">
                        <MarkdownRenderer value={editingBodyMd || "（プレビューはここに表示されます）"} />
                      </div>
                    </TabsContent>
                  </Tabs>
                  {editErrors.bodyMd && <p className="text-xs text-destructive">{editErrors.bodyMd.message}</p>}
                  <div className="flex items-center gap-2">
                    <Button
                      type="button"
                      size="sm"
                      disabled={isSubmitting}
                      onClick={() => handleEditSubmit((values) => onUpdateComment(comment.id, values))()}
                    >
                      {isSubmitting ? "更新中..." : "更新"}
                    </Button>
                    <Button
                      type="button"
                      size="sm"
                      variant="outline"
                      disabled={isSubmitting}
                      onClick={() => {
                        setEditingCommentId(null);
                        resetEdit({ bodyMd: "" });
                      }}
                    >
                      キャンセル
                    </Button>
                  </div>
                </div>
              ) : (
                <MarkdownRenderer value={comment.bodyMd} />
              )}
            </article>
          ))
        )}
      </div>

      <div className="rounded-xl border p-4">
        <Tabs defaultValue="write" className="gap-3">
          <TabsList>
            <TabsTrigger value="write">入力</TabsTrigger>
            <TabsTrigger value="preview">プレビュー</TabsTrigger>
          </TabsList>
          <TabsContent value="write">
            <div className="h-48">
              <MarkdownEditor value={bodyMd} onChangeAction={setBodyMd} />
            </div>
          </TabsContent>
          <TabsContent value="preview">
            <div className="min-h-48 rounded-lg border bg-muted/30 p-2">
              <MarkdownRenderer value={bodyMd || "（プレビューはここに表示されます）"} />
            </div>
          </TabsContent>
        </Tabs>

        <div className="mt-3 flex items-center justify-between gap-3">
          <p className="text-xs text-muted-foreground">{bodyMd.length}/2000</p>
          <Button type="button" size="sm" disabled={isSubmitting} onClick={onSubmit}>
            コメントを投稿
          </Button>
        </div>
        {error && <p className="mt-2 text-sm text-destructive">{error}</p>}
      </div>
    </section>
  );
}
