import { NavLink } from "../../atoms/NavLink";

export type NavItem = { label: string; href?: string; onClick?: () => void };

export function NavList({ items }: { items: NavItem[] }) {
  return (
    <nav className="hidden items-center gap-1 md:flex">
      {items.map((it) =>
        it.href ? (
          <NavLink key={it.href} href={it.href}>
            {it.label}
          </NavLink>
        ) : (
          <button
            onClick={it.onClick}
            key={it.label}
            type="button"
            className="cursor-pointer px-3 py-2 text-sm text-muted-foreground hover:text-foreground transition-colors"
          >
            {it.label}
          </button>
        ),
      )}
    </nav>
  );
}
