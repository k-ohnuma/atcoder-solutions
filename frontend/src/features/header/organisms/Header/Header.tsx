"use client";

import { onAuthStateChanged } from "firebase/auth";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { onSubmitDeleteAccount, onSubmitSignout } from "@/features/auth/lib/submit";
import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { Logo } from "@/shared/ui/atoms/Logo";

export function Header({ appName }: { appName: string }) {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [myUserName, setMyUserName] = useState<string | null>(null);
  const [isMyPageMenuOpen, setIsMyPageMenuOpen] = useState(false);
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
    await onSubmitSignout();
    router.push("/");
  };
  const handleDeleteAccount = async () => {
    const ok = confirm("アカウントを削除しますか？この操作は取り消せません。");
    if (!ok) {
      return;
    }
    try {
      await onSubmitDeleteAccount();
      router.push("/");
    } catch (e) {
      const message =
        e instanceof Error ? e.message : "アカウント削除に失敗しました。時間をおいて再度お試しください。";
      alert(message);
    }
  };

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/80">
      <div className="flex h-14 w-full items-center px-4 md:px-8">
        <Logo className="mr-4" appName={appName} />

        <div className="ml-auto flex items-center gap-2">
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
                        setIsMyPageMenuOpen((prev) => !prev);
                      }}
                    >
                      マイページ
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent
                    align="end"
                    className="w-44"
                    onEscapeKeyDown={() => setIsMyPageMenuOpen(false)}
                    onInteractOutside={() => setIsMyPageMenuOpen(false)}
                  >
                    <DropdownMenuItem asChild>
                      <Link
                        href={`/users/${encodeURIComponent(myUserName)}/solutions`}
                        onClick={() => setIsMyPageMenuOpen(false)}
                      >
                        解説一覧
                      </Link>
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      className="cursor-pointer"
                      onSelect={async () => {
                        setIsMyPageMenuOpen(false);
                        await handleDeleteAccount();
                      }}
                    >
                      退会
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              ) : null}
              <Button
                onClick={handleSignOut}
                type="button"
                variant="ghost"
                size="sm"
              >
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
        </div>
      </div>
    </header>
  );
}
