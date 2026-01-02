"use client";

import React from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/cjs/styles/prism";
import type { Components } from "react-markdown";

type MarkdownRendererProps = {
  value: string;
};

const components: Components = {
  code({ className, children, ...rest }) {
    const match = /language-(\w+)/.exec(className ?? "");
    const code = String(children).replace(/\n$/, "");

    if (match) {
      return (
        <SyntaxHighlighter
          style={oneDark}
          language={match[1]}
          PreTag="div"
          className={className}
        >
          {code}
        </SyntaxHighlighter>
      );
    }
    return (
      <code className={className} {...rest}>
        {children}
      </code>
    );
  },
};

export const MarkdownRenderer: React.FC<MarkdownRendererProps> = ({ value }) => {
  return (
    <div className="prose prose-sm max-w-none prose-slate dark:prose-invert rounded-lg border border-border p-3">
      <ReactMarkdown
        remarkPlugins={[remarkGfm, remarkMath]}
        rehypePlugins={[rehypeKatex]}
        components={components}
      >
        {value}
      </ReactMarkdown>
    </div>
  );
};
