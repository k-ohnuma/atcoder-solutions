"use client";

import Link from "next/link";
import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";

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
        <Button asChild variant="ghost" size="sm">
          <Link href="/recent">最近の記事</Link>
        </Button>
      </nav>
      {isLoggedIn ? (
        <nav className="hidden items-center gap-1 md:flex">
          <Button asChild variant="ghost" size="sm">
            <Link href="/solutions/create">記事を書く</Link>
          </Button>
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
                <DropdownMenuItem asChild>
                  <Link
                    href={`/users/${encodeURIComponent(myUserName)}/solutions`}
                    onClick={() => setIsMyPageMenuOpenAction(false)}
                  >
                    解説一覧
                  </Link>
                </DropdownMenuItem>
                <DropdownMenuItem asChild>
                  <Link href="/settings/password" onClick={() => setIsMyPageMenuOpenAction(false)}>
                    パスワード変更
                  </Link>
                </DropdownMenuItem>
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
          <Button asChild variant="ghost" size="sm">
            <Link href="/signin">ログイン</Link>
          </Button>
          <Button asChild variant="ghost" size="sm">
            <Link href="/signup">新規登録</Link>
          </Button>
        </nav>
      )}
    </>
  );
}
