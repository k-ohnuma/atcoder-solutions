"use client";

import { css } from "styled-system/css";

export function AuthTemplate({ title, children }: { title: string; children: React.ReactNode }) {
  const mainStyle = css({
    maxWidth: "xl",
    px: "4",
    py: "8",
    mx: "auto",
  });
  const titleStyle = css({
    mb: "6",
    fontWeight: "semibold",
    fontSize: "2xl",
  });
  return (
    <main className={mainStyle}>
      <h1 className={titleStyle}>{title}</h1>
      {children}
    </main>
  );
}
