"use client";

import React, { useState } from "react";
import { css } from "styled-system/css";
import { MarkdownEditor } from "../molecules";
import { MarkdownRenderer } from "../atoms";

export const MarkdownPlayground: React.FC = () => {
  const [text, setText] = useState<string>("");

  return (
    <div
      className={css({
        display: "grid",
        gridTemplateColumns: { base: "1fr", md: "1fr 1fr" },
        gap: 4,
        h: "calc(100vh - 80px)",
        p: 4,
      })}
    >
      <div
        className={css({
          borderWidth: 1,
          borderColor: "gray.4",
          borderRadius: "xl",
          p: 4,
          display: "flex",
          flexDir: "column",
        })}
      >
        <MarkdownEditor value={text} onChange={setText} />
      </div>

      <div
        className={css({
          borderWidth: 1,
          borderColor: "gray.4",
          borderRadius: "xl",
          p: 4,
          bg: "gray.1",
        })}
      >
        <MarkdownRenderer value={text} />
      </div>
    </div>
  );
};

