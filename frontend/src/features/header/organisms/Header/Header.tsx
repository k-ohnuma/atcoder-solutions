import { css } from "styled-system/css";
import { Logo } from "@/shared/ui/atoms/Logo";
import { NavList, type NavItem } from "../../molecules/NavList";

const defaultItems: NavItem[] = [
  { href: "/signin", label: "ログイン" },
  { href: "/signup", label: "新規登録" },
];

export function Header({
  appName,
  items = defaultItems,
}: {
  items?: NavItem[];
  appName: string;
}) {
  return (
    <header
      className={css({
        w: "full",
        borderBottomWidth: "1px",
      })}
    >
      <div
        className={css({
          mx: "auto",
          display: "flex",
          h: "14",
          alignItems: "center",
          px: "4",
        })}
      >
        <Logo className={css({ mr: "4" })} appName={appName} />

        <div
          className={css({
            ml: "auto",
            display: "flex",
            alignItems: "center",
            gap: "2",
          })}
        >
          <NavList items={items} />
        </div>
      </div>
    </header>
  );
}
