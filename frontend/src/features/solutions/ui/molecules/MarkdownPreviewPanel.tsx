"use client";

import { MarkdownRenderer } from "../atoms";

type MarkdownPreviewPanelProps = {
  title: string;
  bodyMd: string;
};

export function MarkdownPreviewPanel({ title, bodyMd }: MarkdownPreviewPanelProps) {
  return (
    <div className="flex h-full min-h-[800px] flex-col rounded-xl border bg-muted/40">
      <div className="border-b px-3 py-2">
        <div className="text-xs font-semibold text-muted-foreground">{title || "（タイトル未設定）"}</div>
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        <MarkdownRenderer value={bodyMd} />
      </div>
    </div>
  );
}
