"use client";

import { css } from "styled-system/css";
import { Logo } from "@/shared/ui/atoms/Logo";
import { NavList, type NavItem } from "../../molecules/NavList";
import { useEffect, useState } from "react";
import { getFirebaseAuth } from "@/shared/firebase/client";
import { onAuthStateChanged } from "firebase/auth";
import { onSubmitSignout } from "@/features/auth/lib/submit";
import { useRouter } from "next/navigation";

const nonLoginItems: NavItem[] = [
  { href: "/signin", label: "ログイン" },
  { href: "/signup", label: "新規登録" },
];

export function Header({ appName }: { items?: NavItem[]; appName: string }) {
  // TODO: サーバー側で認証持ちたい
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const auth = getFirebaseAuth();


  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (user) => {
      setIsLoggedIn(!!user);
    });
    return () => unsub();
  }, [auth]);

  const router = useRouter();
  const handleSignOut = async() => {
    await onSubmitSignout();
    router.push('/');

  }
  const loginItems: NavItem[] = [{ label: "ログアウト", onClick: handleSignOut }];
  const items = isLoggedIn ? loginItems : nonLoginItems;
  return (
    <header
      className={css({
        w: "full",
        borderBottomWidth: "1px",
      })}
    >
      <div
        className={css({
          mx: "auto",
          display: "flex",
          h: "14",
          alignItems: "center",
          px: "4",
        })}
      >
        <Logo className={css({ mr: "4" })} appName={appName} />

        <div
          className={css({
            ml: "auto",
            display: "flex",
            alignItems: "center",
            gap: "2",
          })}
        >
          <NavList items={items} />
        </div>
      </div>
    </header>
  );
}
