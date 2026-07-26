"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";
import { RequiresRecentLoginError } from "@/features/auth/lib/errors";
import { onSubmitDeleteAccount, onSubmitSignout } from "@/features/auth/lib/session";

export function useHeaderActions() {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isMyPageMenuOpen, setIsMyPageMenuOpen] = useState(false);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [isDeleteErrorDialogOpen, setIsDeleteErrorDialogOpen] = useState(false);
  const [isRecentLoginDialogOpen, setIsRecentLoginDialogOpen] = useState(false);
  const [deleteErrorMessage, setDeleteErrorMessage] = useState("");
  const [isDeleting, setIsDeleting] = useState(false);
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
      if (e instanceof RequiresRecentLoginError) {
        setIsDeleteDialogOpen(false);
        setIsRecentLoginDialogOpen(true);
        return;
      }
      const message = e instanceof Error ? e.message : "アカウント削除に失敗しました。時間をおいて再度お試しください。";
      setDeleteErrorMessage(message);
      setIsDeleteDialogOpen(false);
      setIsDeleteErrorDialogOpen(true);
    } finally {
      setIsDeleting(false);
    }
  };

  const handleRequireRecentLogin = async () => {
    setIsRecentLoginDialogOpen(false);
    await onSubmitSignout();
    router.push("/signin");
  };

  return {
    isMobileMenuOpen,
    setIsMobileMenuOpen,
    isMyPageMenuOpen,
    setIsMyPageMenuOpen,
    isDeleteDialogOpen,
    setIsDeleteDialogOpen,
    isDeleteErrorDialogOpen,
    setIsDeleteErrorDialogOpen,
    isRecentLoginDialogOpen,
    setIsRecentLoginDialogOpen,
    deleteErrorMessage,
    isDeleting,
    handleSignOut,
    handleDeleteAccount,
    handleRequireRecentLogin,
  };
}
