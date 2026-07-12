"use client";

import { FieldError } from "react-hook-form";
import { MarkdownEditor } from "./MarkdownEditor";
import { MarkdownPreviewPanel } from "./MarkdownPreviewPanel";

type CreateSolutionBodyEditorProps = {
  title: string;
  bodyMd: string;
  error?: FieldError;
  onBodyChangeAction: (value: string) => void;
};

export function CreateSolutionBodyEditor({ title, bodyMd, error, onBodyChangeAction }: CreateSolutionBodyEditorProps) {
  return (
    <div className="grid min-h-[65vh] grid-cols-1 gap-4 md:grid-cols-2">
      <section className="flex min-h-[65vh] flex-col rounded-xl border border-border bg-background">
        <div className="border-b px-3 py-2 text-xs font-semibold text-muted-foreground">本文（Markdown）</div>
        <div className="flex-1 p-3">
          <MarkdownEditor value={bodyMd} onChangeAction={onBodyChangeAction} />
        </div>
        {error && <p className="px-3 pb-3 text-xs text-destructive">{error.message}</p>}
      </section>

      <section className="min-h-[65vh]">
        <MarkdownPreviewPanel title={title} bodyMd={bodyMd} />
      </section>
    </div>
  );
}
