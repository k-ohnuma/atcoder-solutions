"use client";

import { Menu } from "lucide-react";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";
import { authenticatedHeaderNavItems, getMyPageNavItems, guestHeaderNavItems, publicHeaderNavItems } from "./headerNavItems";

type HeaderMobileMenuProps = {
  isOpen: boolean;
  onOpenChangeAction: (open: boolean) => void;
  isAuthResolved: boolean;
  isLoggedIn: boolean;
  myUserName: string | null;
  onOpenDeleteDialogAction: () => void;
  onSignOutAction: () => void;
};

export function HeaderMobileMenu({
  isOpen,
  onOpenChangeAction,
  isAuthResolved,
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
          {publicHeaderNavItems.map((item) => (
            <Button key={item.href} asChild variant="ghost" className="justify-start">
              <Link href={item.href} onClick={() => onOpenChangeAction(false)}>
                {item.label}
              </Link>
            </Button>
          ))}
          {!isAuthResolved ? null : isLoggedIn ? (
            <>
              {authenticatedHeaderNavItems.map((item) => (
                <Button key={item.href} asChild variant="ghost" className="justify-start">
                  <Link href={item.href} onClick={() => onOpenChangeAction(false)}>
                    {item.label}
                  </Link>
                </Button>
              ))}
              {myUserName
                ? getMyPageNavItems(myUserName).map((item) => (
                    <Button key={item.href} asChild variant="ghost" className="justify-start">
                      <Link href={item.href} onClick={() => onOpenChangeAction(false)}>
                        {item.label}
                      </Link>
                    </Button>
                  ))
                : null}
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
            guestHeaderNavItems.map((item) => (
              <Button key={item.href} asChild variant="ghost" className="justify-start">
                <Link href={item.href} onClick={() => onOpenChangeAction(false)}>
                  {item.label}
                </Link>
              </Button>
            ))
          )}
        </div>
      </SheetContent>
    </Sheet>
  );
}
