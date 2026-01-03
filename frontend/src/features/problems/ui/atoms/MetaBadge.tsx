import React from "react";
import { Badge } from "@/components/ui/badge";

export const MetaBadge = ({ children, title }: { children: React.ReactNode; title?: string }) => {
  return (
    <span title={title} className="inline-flex text-sm">
      <Badge>{children}</Badge>
    </span>
  );
};
