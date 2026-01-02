"use client";

import { onAuthStateChanged } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { onSubmitSignout } from "@/features/auth/lib/submit";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { Logo } from "@/shared/ui/atoms/Logo";
import { type NavItem, NavList } from "../../molecules/NavList";

const nonLoginItems: NavItem[] = [
  { href: "/signin", label: "ログイン" },
  { href: "/signup", label: "新規登録" },
];

export function Header({ appName }: { items?: NavItem[]; appName: string }) {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const auth = getFirebaseAuth();

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      setIsLoggedIn(!!user);
    });
    return () => unsub();
  }, [auth]);

  const router = useRouter();
  const handleSignOut = async () => {
    await onSubmitSignout();
    router.push("/");
  };

  const loginItems: NavItem[] = [{ label: "ログアウト", onClick: handleSignOut }];
  const items = isLoggedIn ? loginItems : nonLoginItems;

  return (
    <header className="w-full border-b">
      <div className="mx-auto flex h-14 items-center px-4">
        <Logo className="mr-4" appName={appName} />

        <div className="ml-auto flex items-center gap-2">
          <NavList items={items} />
        </div>
      </div>
    </header>
  );
}
