"use client";

import React from "react";
import type { Components } from "react-markdown";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/cjs/styles/prism";
import rehypeKatex from "rehype-katex";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import { cn } from "@/lib/utils";

type MarkdownRendererProps = {
  value: string;
};

const ListDepthContext = React.createContext(0);

const components: Components = {
  ul({ className, children, ...rest }) {
    const depth = React.useContext(ListDepthContext) + 1;
    const markerClass = depth === 1 ? "list-disc" : depth === 2 ? "list-[circle]" : "list-[square]";

    return (
      <ListDepthContext.Provider value={depth}>
        <ul className={cn(markerClass, "pl-6", className)} {...rest}>
          {children}
        </ul>
      </ListDepthContext.Provider>
    );
  },
  code({ className, children, ...rest }) {
    const match = /language-(\w+)/.exec(className ?? "");
    const code = String(children).replace(/\n$/, "");

    if (match) {
      return (
        <SyntaxHighlighter style={oneDark} language={match[1]} PreTag="div" className={className}>
          {code}
        </SyntaxHighlighter>
      );
    }
    return (
      <code className={`rounded bg-muted px-1 py-0.5 font-mono text-[0.95em] font-normal ${className ?? ""}`.trim()} {...rest}>
        {children}
      </code>
    );
  },
};

export const MarkdownRenderer: React.FC<MarkdownRendererProps> = ({ value }) => {
  return (
    <ListDepthContext.Provider value={0}>
      <div className="prose prose-zinc max-w-none px-3 py-1 leading-7 md:px-5 md:py-2 dark:prose-invert prose-headings:scroll-mt-24 prose-headings:font-semibold prose-h2:mt-10 prose-h2:mb-4 prose-h3:mt-8 prose-h3:mb-3 prose-p:my-4 prose-p:whitespace-pre-wrap prose-li:my-1.5 prose-pre:rounded-xl prose-pre:border prose-pre:px-4 prose-pre:py-3 prose-code:before:content-none prose-code:after:content-none prose-table:text-sm">
        <ReactMarkdown remarkPlugins={[remarkGfm, remarkMath]} rehypePlugins={[rehypeKatex]} components={components}>
          {value}
        </ReactMarkdown>
      </div>
    </ListDepthContext.Provider>
  );
};
