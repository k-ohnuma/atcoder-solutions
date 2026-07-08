"use client";

import Image from "next/image";
import Link from "next/link";
import { useCurrentUser } from "@/features/auth/hooks/useCurrentUser";
import { DeleteAccountDialogs } from "./DeleteAccountDialogs";
import { HeaderDesktopNav } from "./HeaderDesktopNav";
import { HeaderMobileMenu } from "./HeaderMobileMenu";
import { useHeaderActions } from "./useHeaderActions";

export function Header({ appName }: { appName: string }) {
  const { isLoggedIn, myUserName } = useCurrentUser();
  const {
    isMobileMenuOpen,
    setIsMobileMenuOpen,
    isMyPageMenuOpen,
    setIsMyPageMenuOpen,
    isDeleteDialogOpen,
    setIsDeleteDialogOpen,
    isDeleteErrorDialogOpen,
    setIsDeleteErrorDialogOpen,
    deleteErrorMessage,
    isDeleting,
    handleSignOut,
    handleDeleteAccount,
  } = useHeaderActions();

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/80">
      <div className="flex h-14 w-full items-center px-4 md:px-8">
        <Link href="/" className="mr-4 inline-flex items-center gap-2 font-semibold">
          <Image src="/icon.svg" alt={appName} width={20} height={20} />
          <span>{appName}</span>
        </Link>

        <div className="ml-auto flex items-center gap-2">
          <HeaderMobileMenu
            isOpen={isMobileMenuOpen}
            onOpenChangeAction={setIsMobileMenuOpen}
            isLoggedIn={isLoggedIn}
            myUserName={myUserName}
            onOpenDeleteDialogAction={() => setIsDeleteDialogOpen(true)}
            onSignOutAction={handleSignOut}
          />
          <HeaderDesktopNav
            isLoggedIn={isLoggedIn}
            myUserName={myUserName}
            isMyPageMenuOpen={isMyPageMenuOpen}
            setIsMyPageMenuOpenAction={setIsMyPageMenuOpen}
            onOpenDeleteDialogAction={() => setIsDeleteDialogOpen(true)}
            onSignOutAction={handleSignOut}
          />
        </div>
      </div>
      <DeleteAccountDialogs
        isDeleteDialogOpen={isDeleteDialogOpen}
        onDeleteDialogOpenChangeAction={setIsDeleteDialogOpen}
        isDeleteErrorDialogOpen={isDeleteErrorDialogOpen}
        onDeleteErrorDialogOpenChangeAction={setIsDeleteErrorDialogOpen}
        deleteErrorMessage={deleteErrorMessage}
        isDeleting={isDeleting}
        onDeleteAccountAction={handleDeleteAccount}
      />
    </header>
  );
}
