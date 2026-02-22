import Link from "next/link";
import { PageContainer } from "@/components/layout/PageContainer";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";

export default function NotFoundPage() {
  return (
    <PageContainer>
      <Card className="mx-auto mt-12 w-full max-w-2xl">
        <CardHeader>
          <CardTitle>ページが見つかりません</CardTitle>
          <CardDescription>URLが間違っているか、ページが移動または削除された可能性があります。</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">トップページから問題一覧や解説一覧に移動してください。</p>
        </CardContent>
        <CardFooter>
          <Button asChild>
            <Link href="/">トップへ戻る</Link>
          </Button>
        </CardFooter>
      </Card>
    </PageContainer>
  );
}
