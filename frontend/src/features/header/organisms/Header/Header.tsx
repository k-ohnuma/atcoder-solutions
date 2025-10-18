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
    <header className="w-full border-b">
      <div className="mx-auto flex h-14 max-w-6xl items-center px-4">
        <Logo className="mr-4" appName={appName} />

        <div className="ml-auto flex items-center gap-2">
          <NavList items={items} />
        </div>
      </div>
    </header>
  );
}
