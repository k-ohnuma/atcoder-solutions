"use client";

import { Button } from "@/components/ui/button";
import type { Problem } from "@/shared/model/problem";

type CreateSolutionHeaderProps = {
  title: string;
  selectedProblem: Problem | null;
  submitError: string | null;
  isSubmitting: boolean;
};

export function CreateSolutionHeader({ title, selectedProblem, submitError, isSubmitting }: CreateSolutionHeaderProps) {
  return (
    <div className="sticky top-16 z-20 rounded-xl border border-border bg-background/95 p-3 backdrop-blur">
      <div className="flex items-center justify-between gap-3">
        <div className="min-w-0">
          <p className="truncate text-sm font-semibold">{title || "新規解説"}</p>
          <p className="text-xs text-muted-foreground">
            {selectedProblem ? `${selectedProblem.id} の解説` : "問題と内容を入力して投稿"}
          </p>
        </div>
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting ? "送信中..." : "投稿する"}
        </Button>
      </div>
      {submitError && <p className="mt-2 text-sm text-destructive">{submitError}</p>}
    </div>
  );
}
