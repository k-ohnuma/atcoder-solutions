import Link from "next/link";
import { css, cx } from "styled-system/css";

type Props = { href?: string; className?: string; appName: string };
export function Logo({ href = "/", className, appName }: Props) {
  const wrapper = css({
    display: "inline-flex",
    alignItems: "center",
    gap: "2",
    fontWeight: "semibold",
    verticalAlign: "middle",
  });

  const icon = css({
    display: "inline-block",
    h: "5",
    w: "5",
    rounded: "md",
    bg: "beige",
  });
  const inner = (
    <span className={cx(wrapper, className)}>
      <span className={icon} />
      <span>{appName}</span>
    </span>
  );
  return href ? <Link href={href}>{inner}</Link> : inner;
}
