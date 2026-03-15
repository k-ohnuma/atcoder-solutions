"use client";

import { Menu } from "lucide-react";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";

type HeaderMobileMenuProps = {
  isOpen: boolean;
  onOpenChangeAction: (open: boolean) => void;
  isLoggedIn: boolean;
  myUserName: string | null;
  onOpenDeleteDialogAction: () => void;
  onSignOutAction: () => void;
};

export function HeaderMobileMenu({
  isOpen,
  onOpenChangeAction,
  isLoggedIn,
  myUserName,
  onOpenDeleteDialogAction,
  onSignOutAction,
}: HeaderMobileMenuProps) {
  return (
    <Sheet open={isOpen} onOpenChange={onOpenChangeAction}>
      <SheetTrigger asChild>
        <Button type="button" variant="ghost" size="icon" className="md:hidden" aria-label="メニューを開く">
          <Menu className="size-5" />
        </Button>
      </SheetTrigger>
      <SheetContent side="right" className="w-[20rem] p-0">
        <SheetHeader className="border-b px-6 py-5 text-left">
          <SheetTitle>メニュー</SheetTitle>
        </SheetHeader>
        <div className="flex flex-col gap-2 p-4">
          <Button asChild variant="ghost" className="justify-start">
            <Link href="/recent" onClick={() => onOpenChangeAction(false)}>
              最近の記事
            </Link>
          </Button>
          {isLoggedIn ? (
            <>
              <Button asChild variant="ghost" className="justify-start">
                <Link href="/solutions/create" onClick={() => onOpenChangeAction(false)}>
                  記事を書く
                </Link>
              </Button>
              {myUserName ? (
                <>
                  <Button asChild variant="ghost" className="justify-start">
                    <Link href={`/users/${encodeURIComponent(myUserName)}/solutions`} onClick={() => onOpenChangeAction(false)}>
                      解説一覧
                    </Link>
                  </Button>
                  <Button asChild variant="ghost" className="justify-start">
                    <Link href="/settings/password" onClick={() => onOpenChangeAction(false)}>
                      パスワード変更
                    </Link>
                  </Button>
                </>
              ) : null}
              <Button
                type="button"
                variant="ghost"
                className="justify-start"
                onClick={() => {
                  onOpenChangeAction(false);
                  onOpenDeleteDialogAction();
                }}
              >
                退会
              </Button>
              <Button type="button" variant="ghost" className="justify-start" onClick={onSignOutAction}>
                ログアウト
              </Button>
            </>
          ) : (
            <>
              <Button asChild variant="ghost" className="justify-start">
                <Link href="/signin" onClick={() => onOpenChangeAction(false)}>
                  ログイン
                </Link>
              </Button>
              <Button asChild variant="ghost" className="justify-start">
                <Link href="/signup" onClick={() => onOpenChangeAction(false)}>
                  新規登録
                </Link>
              </Button>
            </>
          )}
        </div>
      </SheetContent>
    </Sheet>
  );
}
