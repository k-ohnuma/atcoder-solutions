"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

type Props = {
  href: string;
  children: React.ReactNode;
  exact?: boolean;
  className?: string;
};

export function NavLink({ href, children, exact = false, className }: Props) {
  const pathname = usePathname();
  const active = exact ? pathname === href : pathname?.startsWith(href);
  return (
    <Link
      href={href}
      className={[
        "px-3 py-2 text-sm transition-colors",
        active ? "text-foreground" : "text-muted-foreground hover:text-foreground",
        className,
      ].filter(Boolean).join(" ")}
    >
      {children}
    </Link>
  );
}

