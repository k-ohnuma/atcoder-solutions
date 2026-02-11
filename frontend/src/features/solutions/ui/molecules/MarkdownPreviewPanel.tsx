"use client";

import { MarkdownRenderer } from "../atoms";

type MarkdownPreviewPanelProps = {
  title: string;
  bodyMd: string;
  contestId: string | null;
  selectedProblemIndex?: string;
  selectedProblemTitle?: string;
};

export function MarkdownPreviewPanel({
  title,
  bodyMd,
  contestId,
  selectedProblemIndex,
  selectedProblemTitle,
}: MarkdownPreviewPanelProps) {
  return (
    <div className="flex h-full min-h-[800px] flex-col rounded-xl border border-border bg-muted/40">
      <div className="border-b px-3 py-2">
        <div className="text-sm font-semibold">{title || "（タイトル未設定）"}</div>
        {selectedProblemIndex && selectedProblemTitle && (
          <div className="text-xs text-muted-foreground">
            {contestId?.toUpperCase()} / {selectedProblemIndex}: {selectedProblemTitle}
          </div>
        )}
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        <MarkdownRenderer value={bodyMd} />
      </div>
    </div>
  );
}
