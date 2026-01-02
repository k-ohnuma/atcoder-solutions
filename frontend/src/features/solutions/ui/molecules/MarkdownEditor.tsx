"use client";

import React from "react";

type MarkdownEditorProps = {
  value: string;
  onChangeAction: (value: string) => void;
};

export const MarkdownEditor: React.FC<MarkdownEditorProps> = ({ value, onChangeAction }) => {
  return (
    <div className="flex h-full w-full flex-col">
      <textarea
        className="flex-1 resize-none rounded-lg border border-border p-3 font-mono text-sm outline-none focus-visible:ring-1 focus-visible:ring-ring"
        value={value}
        onChange={(e) => onChangeAction(e.target.value)}
      />
    </div>
  );
};
