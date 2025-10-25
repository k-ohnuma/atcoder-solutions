import { css } from "styled-system/css";
import { NavLink } from "../../atoms/NavLink";

export type NavItem = { label: string; href?: string; onClick?: () => void };

export function NavList({ items }: { items: NavItem[] }) {
  return (
    <nav
      className={css({
        // display: "none",
        md: { display: "flex" },
        alignItems: "center",
        gap: "1",
      })}
    >
      {items.map((it) =>
        it.href ? (
          <NavLink key={it.href} href={it.href}>
            {it.label}
          </NavLink>
        ) : (
          <button
            onClick={it.onClick}
            key={it.label}
            className={css({
              px: 3,
              py: 2,
              fontSize: "sm",
              color: "fg.muted",
              cursor: "pointer",
            })}
          >
            {it.label}
          </button>
        ),
      )}
    </nav>
  );
}
