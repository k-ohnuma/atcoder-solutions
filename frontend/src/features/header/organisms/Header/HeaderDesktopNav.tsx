"use client";

import Link from "next/link";
import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import { authenticatedHeaderNavItems, getMyPageNavItems, guestHeaderNavItems, publicHeaderNavItems } from "./headerNavItems";

type HeaderDesktopNavProps = {
  isLoggedIn: boolean;
  myUserName: string | null;
  isMyPageMenuOpen: boolean;
  setIsMyPageMenuOpenAction: (open: boolean) => void;
  onOpenDeleteDialogAction: () => void;
  onSignOutAction: () => void;
};

export function HeaderDesktopNav({
  isLoggedIn,
  myUserName,
  isMyPageMenuOpen,
  setIsMyPageMenuOpenAction,
  onOpenDeleteDialogAction,
  onSignOutAction,
}: HeaderDesktopNavProps) {
  return (
    <>
      <nav className="hidden items-center gap-1 md:flex">
        {publicHeaderNavItems.map((item) => (
          <Button key={item.href} asChild variant="ghost" size="sm">
            <Link href={item.href}>{item.label}</Link>
          </Button>
        ))}
      </nav>
      {isLoggedIn ? (
        <nav className="hidden items-center gap-1 md:flex">
          {authenticatedHeaderNavItems.map((item) => (
            <Button key={item.href} asChild variant="ghost" size="sm">
              <Link href={item.href}>{item.label}</Link>
            </Button>
          ))}
          {myUserName ? (
            <DropdownMenu open={isMyPageMenuOpen} modal={false}>
              <DropdownMenuTrigger asChild>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  onClick={() => {
                    setIsMyPageMenuOpenAction(!isMyPageMenuOpen);
                  }}
                >
                  マイページ
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                align="end"
                className="w-44"
                onEscapeKeyDown={() => setIsMyPageMenuOpenAction(false)}
                onInteractOutside={() => setIsMyPageMenuOpenAction(false)}
              >
                {getMyPageNavItems(myUserName).map((item) => (
                  <DropdownMenuItem key={item.href} asChild>
                    <Link href={item.href} onClick={() => setIsMyPageMenuOpenAction(false)}>
                      {item.label}
                    </Link>
                  </DropdownMenuItem>
                ))}
                <DropdownMenuItem
                  className="cursor-pointer"
                  onSelect={async () => {
                    setIsMyPageMenuOpenAction(false);
                    onOpenDeleteDialogAction();
                  }}
                >
                  退会
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          ) : null}
          <Button onClick={onSignOutAction} type="button" variant="ghost" size="sm">
            ログアウト
          </Button>
        </nav>
      ) : (
        <nav className="hidden items-center gap-1 md:flex">
          {guestHeaderNavItems.map((item) => (
            <Button key={item.href} asChild variant="ghost" size="sm">
              <Link href={item.href}>{item.label}</Link>
            </Button>
          ))}
        </nav>
      )}
    </>
  );
}
