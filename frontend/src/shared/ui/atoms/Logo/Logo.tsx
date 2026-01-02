import Link from "next/link";
import { cn } from "@/lib/utils";

type Props = { href?: string; className?: string; appName: string };

export function Logo({ href = "/", className, appName }: Props) {
  const inner = (
    <span className={cn("inline-flex items-center gap-2 align-middle font-semibold", className)}>
      <span className="inline-block h-5 w-5 rounded-md bg-muted" />
      <span>{appName}</span>
    </span>
  );

  return href ? <Link href={href}>{inner}</Link> : inner;
}
