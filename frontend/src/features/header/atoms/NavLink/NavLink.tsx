"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cva, cx } from "styled-system/css";

type Props = {
  href: string;
  children: React.ReactNode;
  exact?: boolean;
  className?: string;
};

const linkStyle = cva({
  base: {
    px: "3",
    py: "2",
    fontSize: "sm",
    transitionProperty: "colors",
    transitionDuration: "200ms",
  },
  variants: {
    active: {
      true: { color: "fg" },
      false: { color: "fg.muted", _hover: { color: "fg" } },
    },
  },
});

export function NavLink({ href, children, exact = false, className }: Props) {
  const pathname = usePathname();
  const active = exact ? pathname === href : pathname?.startsWith(href);

  return (
    <Link href={href} className={cx(linkStyle({ active }), className)}>
      {children}
    </Link>
  );
}
