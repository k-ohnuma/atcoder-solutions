"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ApiClient } from "@/lib/client/apiClient";
import { getFirebaseIdToken } from "@/lib/client/firebaseToken";
import { SolutionComment } from "@/shared/model/solution";
import { MarkdownRenderer } from "../atoms";
import { MarkdownEditor } from "../molecules";

const apiClient = new ApiClient();

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
  const [comments, setComments] = useState(initialComments);
  const [bodyMd, setBodyMd] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

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
              </div>
              <MarkdownRenderer value={comment.bodyMd} />
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
