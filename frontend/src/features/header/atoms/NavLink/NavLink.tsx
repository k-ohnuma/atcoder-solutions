"use client";

import { cn } from "@/lib/utils";
import { cva } from "class-variance-authority";
import Link from "next/link";
import { usePathname } from "next/navigation";

type Props = {
  href: string;
  children: React.ReactNode;
  exact?: boolean;
  className?: string;
};

const linkStyle = cva("px-3 py-2 text-sm transition-colors duration-200", {
  variants: {
    active: {
      true: "text-foreground",
      false: "text-muted-foreground hover:text-foreground",
    },
  },
  defaultVariants: {
    active: false,
  },
});

export function NavLink({ href, children, exact = false, className }: Props) {
  const pathname = usePathname();
  const active = exact ? pathname === href : pathname?.startsWith(href);

  return (
    <Link href={href} className={cn(linkStyle({ active }), className)}>
      {children}
    </Link>
  );
}
