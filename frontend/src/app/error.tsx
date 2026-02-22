"use client";

import Link from "next/link";
import { useEffect } from "react";
import { PageContainer } from "@/components/layout/PageContainer";
import { Button } from "@/components/ui/button";
import { Card, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";

type ErrorPageProps = {
  error: Error & { digest?: string };
};

export default function ErrorPage({ error }: ErrorPageProps) {
  useEffect(() => {
    console.error("app error boundary", { message: error.message, digest: error.digest });
  }, [error]);

  return (
    <PageContainer>
      <Card className="mx-auto mt-12 w-full max-w-2xl">
        <CardHeader>
          <CardTitle>エラーが発生しました</CardTitle>
          <CardDescription>時間をおいて再度お試しください。解決しない場合はトップページへ戻ってください。</CardDescription>
        </CardHeader>
        <CardFooter className="gap-2">
          <Button asChild variant="outline">
            <Link href="/">トップへ戻る</Link>
          </Button>
        </CardFooter>
      </Card>
    </PageContainer>
  );
}
