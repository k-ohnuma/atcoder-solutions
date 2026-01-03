import type { Metadata } from "next";
import "./globals.css";
import "katex/dist/katex.min.css";
import { Header } from "@/features/header/organisms/Header/Header";
import { clientConfig } from "@/shared/config/client";
import { Providers } from "./providers";

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
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
