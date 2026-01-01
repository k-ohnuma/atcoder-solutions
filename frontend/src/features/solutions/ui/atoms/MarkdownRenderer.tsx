"use client";

import React from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { css } from "styled-system/css";

type MarkdownRendererProps = {
  value: string;
};

export const MarkdownRenderer: React.FC<MarkdownRendererProps> = ({ value }) => {
  return (
    <div
      className={css({
        w: "full",
        h: "full",
        overflowY: "auto",
        fontSize: "sm",
        // prose 的な感じを自作
        "& h1": { fontSize: "2xl", fontWeight: "bold", mt: 4, mb: 2 },
        "& h2": { fontSize: "xl", fontWeight: "bold", mt: 4, mb: 2 },
        "& h3": { fontSize: "lg", fontWeight: "semibold", mt: 3, mb: 1 },
        "& p": { mt: 2, lineHeight: "1.8" },
        "& ul": { pl: 4, mt: 2, listStyleType: "disc" },
        "& ol": { pl: 4, mt: 2, listStyleType: "decimal" },
        "& code": {
          px: 1,
          py: 0.5,
          borderRadius: "md",
          bg: "gray.2",
          fontFamily: "mono",
          fontSize: "xs",
        },
        "& pre": {
          mt: 3,
          p: 3,
          borderRadius: "lg",
          bg: "gray.2",
          fontFamily: "mono",
          fontSize: "xs",
          overflowX: "auto",
        },
        "& a": {
          color: "blue.9",
          textDecoration: "underline",
        },
        "& table": {
          mt: 3,
          borderCollapse: "collapse",
          w: "full",
        },
        "& th, & td": {
          borderWidth: 1,
          borderColor: "gray.4",
          p: 2,
          textAlign: "left",
        },
      })}
    >
      <ReactMarkdown remarkPlugins={[remarkGfm]}>
        {value}
      </ReactMarkdown>
    </div>
  );
};

