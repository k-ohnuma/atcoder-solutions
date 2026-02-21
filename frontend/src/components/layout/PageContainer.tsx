import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

type PageContainerProps = {
  children: ReactNode;
  className?: string;
  as?: "main" | "div" | "article" | "section";
};

export function PageContainer({ children, className, as = "main" }: PageContainerProps) {
  const Comp = as;

  return <Comp className={cn("mx-auto w-full max-w-6xl px-4 py-6 md:px-8 md:py-10", className)}>{children}</Comp>;
}
