import type { Metadata } from "next";
import "./globals.css";
import { clientConfig } from "@/shared/config/client";
import { Header } from "@/features/header/organisms/Header/Header";

export const metadata: Metadata = {
  title: clientConfig.appConfig.appName,
  description: clientConfig.appConfig.appName,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body>
        <Header appName={clientConfig.appConfig.appName} />
        {children}
      </body>
    </html>
  );
}
