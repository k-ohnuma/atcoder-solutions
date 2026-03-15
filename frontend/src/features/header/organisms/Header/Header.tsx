"use client";

import { onAuthStateChanged } from "firebase/auth";
import Image from "next/image";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { onSubmitDeleteAccount, onSubmitSignout } from "@/features/auth/lib/submit";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { DeleteAccountDialogs } from "./DeleteAccountDialogs";
import { HeaderDesktopNav } from "./HeaderDesktopNav";
import { HeaderMobileMenu } from "./HeaderMobileMenu";

export function Header({ appName }: { appName: string }) {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [myUserName, setMyUserName] = useState<string | null>(null);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isMyPageMenuOpen, setIsMyPageMenuOpen] = useState(false);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [isDeleteErrorDialogOpen, setIsDeleteErrorDialogOpen] = useState(false);
  const [deleteErrorMessage, setDeleteErrorMessage] = useState("");
  const [isDeleting, setIsDeleting] = useState(false);
  const auth = getFirebaseAuth();

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, async (user) => {
      setIsLoggedIn(!!user);
      if (!user) {
        setMyUserName(null);
        return;
      }
      try {
        const token = await user.getIdToken();
        const res = await fetch("/api/users/me", {
          method: "GET",
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });
        const json = await res.json();
        if (res.ok && json?.ok && json?.data?.userName) {
          setMyUserName(json.data.userName);
          return;
        }
        setMyUserName(null);
      } catch {
        setMyUserName(null);
      }
    });
    return () => unsub();
  }, [auth]);

  const router = useRouter();

  const handleSignOut = async () => {
    setIsMobileMenuOpen(false);
    await onSubmitSignout();
    router.push("/");
  };
  const handleDeleteAccount = async () => {
    setIsDeleting(true);
    try {
      await onSubmitDeleteAccount();
      setIsDeleteDialogOpen(false);
      router.push("/");
    } catch (e) {
      const message = e instanceof Error ? e.message : "アカウント削除に失敗しました。時間をおいて再度お試しください。";
      setDeleteErrorMessage(message);
      setIsDeleteDialogOpen(false);
      setIsDeleteErrorDialogOpen(true);
    } finally {
      setIsDeleting(false);
    }
  };

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
