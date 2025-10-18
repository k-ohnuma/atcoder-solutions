import Link from "next/link";

type Props = { href?: string; className?: string, appName: string };
export function Logo({ href = "/", className, appName }: Props) {
  const inner = (
    <span
      className={["inline-flex items-center gap-2 font-semibold", className]
        .filter(Boolean)
        .join(" ")}
    >
      <span className="inline-block h-5 w-5 rounded bg-primary" />
      <span>{appName}</span>
    </span>
  );
  return href ? <Link href={href}>{inner}</Link> : inner;
}
