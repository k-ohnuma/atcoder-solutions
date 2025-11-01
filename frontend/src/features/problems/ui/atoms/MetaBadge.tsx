import { Badge } from "@/components/ui/badge";
import React from "react";
import { css } from "styled-system/css";

export const MetaBadge = ({
  children,
  title,
}: {
  children: React.ReactNode;
  title?: string;
}) => {
  return (
    <span
      title={title}
      className={css({ display: "inline-flex", fontSize: "sm" })}
    >
      <Badge>{children}</Badge>
    </span>
  );
};
