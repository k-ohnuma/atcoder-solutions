import { PageContainer } from "@/components/layout/PageContainer";

const sections = [
  {
    title: "1. 取得する情報",
    body: [
      "当サイトは、アカウント登録・ログイン・投稿・コメント・いいね等の機能提供のために、ユーザー名、認証に必要な識別情報、投稿内容、コメント内容、いいね等の利用情報を取得します。",
      "また、サービスの安定運用および障害調査のため、アクセスログ、リクエスト情報、エラーログ等を取得する場合があります。",
    ],
  },
  {
    title: "2. 利用目的",
    body: [
      "取得した情報は、本人確認、アカウント管理、解説投稿機能の提供、コメント・いいね機能の提供、不正利用防止、障害対応、サービス改善のために利用します。",
    ],
  },
  {
    title: "3. 外部サービスの利用",
    body: [
      "当サイトは、認証、ホスティング、データ保存、ログ管理等のために、Firebase、Google Cloud、Neon その他の外部サービスを利用する場合があります。",
      "これらの外部サービスにおいて、当サイト運営に必要な範囲で情報が取り扱われることがあります。",
    ],
  },
  {
    title: "4. 第三者提供",
    body: ["法令に基づく場合を除き、ユーザーの個人情報を第三者に提供しません。"],
  },
  {
    title: "5. 公開される情報",
    body: [
      "ユーザー名、投稿した解説、コメント等、サービスの性質上公開される情報があります。公開される内容を確認のうえご利用ください。",
    ],
  },
  {
    title: "6. 保存期間と削除",
    body: [
      "当サイトは、サービス提供に必要な期間、情報を保存します。アカウント削除機能または運営判断により、保存情報の全部または一部を削除することがあります。",
    ],
  },
  {
    title: "7. 安全管理",
    body: [
      "当サイトは、不正アクセス、漏えい、改ざん等を防止するため、合理的な範囲で安全管理措置を講じます。ただし、完全な安全性を保証するものではありません。",
    ],
  },
  {
    title: "8. 改定",
    body: ["本ポリシーは、必要に応じて改定することがあります。重要な変更がある場合は、当サイト上で告知します。"],
  },
  {
    title: "9. お問い合わせ",
    body: ["本ポリシーに関するお問い合わせは、運営者が別途定める連絡先または公開している窓口からご連絡ください。"],
  },
] as const;

export default function PrivacyPage() {
  return (
    <PageContainer as="article" className="max-w-3xl space-y-8">
      <header className="space-y-2">
        <h1 className="text-3xl font-bold">プライバシーポリシー</h1>
        <p className="text-sm text-muted-foreground">最終更新日: 2026-03-12</p>
      </header>

      <p className="text-sm leading-7 text-muted-foreground">
        本プライバシーポリシーは、AtCoder Solutions（以下「当サイト」）におけるユーザー情報の取扱いについて定めるものです。
      </p>

      {sections.map((section) => (
        <section key={section.title} className="space-y-3">
          <h2 className="text-xl font-semibold">{section.title}</h2>
          {section.body.map((paragraph) => (
            <p key={paragraph} className="text-sm leading-7 text-muted-foreground">
              {paragraph}
            </p>
          ))}
        </section>
      ))}
    </PageContainer>
  );
}
