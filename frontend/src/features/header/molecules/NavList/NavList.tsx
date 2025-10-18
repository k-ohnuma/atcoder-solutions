import { NavLink } from "../../atoms/NavLink";

export type NavItem = { href: string; label: string; exact?: boolean };
export function NavList({ items }: { items: NavItem[] }) {
  return (
    <nav className="hidden md:flex items-center gap-1">
      {items.map((it) => (
        <NavLink key={it.href} href={it.href} exact={it.exact}>
          {it.label}
        </NavLink>
      ))}
    </nav>
  );
}
