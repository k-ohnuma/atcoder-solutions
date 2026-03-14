import type { Metadata } from "next";
import "./globals.css";
import "katex/dist/katex.min.css";
import { Header } from "@/features/header/organisms/Header/Header";
import { clientConfig } from "@/shared/config/client";
import { Providers } from "./providers";

export const metadata: Metadata = {
  title: clientConfig.appConfig.appName,
  description: clientConfig.appConfig.appName,
  icons: {
    icon: "/icon.svg",
    shortcut: "/icon.svg",
  },
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
        <Providers>
          {children}
          <footer className="border-t bg-background">
            <div className="flex flex-wrap items-center justify-center gap-4 px-4 py-6 text-sm text-muted-foreground md:px-8">
              <a href="/privacy" className="hover:text-foreground hover:underline">
                プライバシーポリシー
              </a>
              <a href="/terms" className="hover:text-foreground hover:underline">
                利用規約
              </a>
              <a
                href="https://github.com/k-ohnuma/atcoder-solutions"
                target="_blank"
                rel="noreferrer"
                className="hover:text-foreground hover:underline"
              >
                GitHub
              </a>
            </div>
          </footer>
        </Providers>
      </body>
    </html>
  );
}
