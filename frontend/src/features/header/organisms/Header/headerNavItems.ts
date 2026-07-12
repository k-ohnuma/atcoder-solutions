export type HeaderNavItem = {
  label: string;
  href: string;
};

export const publicHeaderNavItems: HeaderNavItem[] = [{ label: "最近の記事", href: "/recent" }];

export const authenticatedHeaderNavItems: HeaderNavItem[] = [{ label: "記事を書く", href: "/solutions/create" }];

export const guestHeaderNavItems: HeaderNavItem[] = [
  { label: "ログイン", href: "/signin" },
  { label: "新規登録", href: "/signup" },
];

export function getMyPageNavItems(userName: string): HeaderNavItem[] {
  const encodedUserName = encodeURIComponent(userName);
  return [
    { label: "解説一覧", href: `/users/${encodedUserName}/solutions` },
    { label: "パスワード変更", href: "/settings/password" },
  ];
}
