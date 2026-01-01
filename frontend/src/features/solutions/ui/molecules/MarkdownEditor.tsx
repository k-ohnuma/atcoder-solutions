"use client";

import React from "react";
import { css } from "styled-system/css";

type MarkdownEditorProps = {
  value: string;
  onChange: (value: string) => void;
};

export const MarkdownEditor: React.FC<MarkdownEditorProps> = ({
  value,
  onChange,
}) => {
  return (
    <div className={css({ w: "full", h: "full", display: "flex", flexDir: "column" })}>
      <textarea
        className={css({
          flex: 1,
          borderWidth: 1,
          borderColor: "gray.4",
          borderRadius: "lg",
          p: 3,
          fontFamily: "mono",
          fontSize: "sm",
          outline: "none",
          _focus: { borderColor: "blue.6", boxShadow: "0 0 0 1px token(colors.blue.6)" },
        })}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={`# タイトル

- リスト
- **太字**
- [リンク](https://example.com)
`}
      />
    </div>
  );
};

