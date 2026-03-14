import { PageContainer } from "@/components/layout/PageContainer";

const sections = [
  {
    title: "1. 適用",
    body: [
      "本利用規約は、AtCoder Solutions（以下「当サイト」）の利用条件を定めるものです。ユーザーは、本規約に同意のうえ当サイトを利用するものとします。",
    ],
  },
  {
    title: "2. アカウント",
    body: [
      "ユーザーは、自己の責任でアカウントを管理するものとします。認証情報の管理不十分により生じた損害について、当サイトは責任を負いません。",
    ],
  },
  {
    title: "3. 投稿内容",
    body: [
      "ユーザーは、自らの責任において解説、コメントその他の内容を投稿するものとします。",
      "投稿内容について、法令違反、第三者権利侵害、公序良俗違反、スパム、不正利用を認めた場合、当サイトは事前通知なく削除その他必要な対応を行うことができます。",
    ],
  },
  {
    title: "4. 禁止事項",
    body: [
      "ユーザーは、法令違反行為、不正アクセス、サービス運営妨害、他者へのなりすまし、過度な自動化、第三者権利侵害、その他運営が不適切と判断する行為をしてはなりません。",
    ],
  },
  {
    title: "5. サービスの変更・停止",
    body: [
      "当サイトは、保守、障害対応、仕様変更その他の理由により、事前通知なくサービス内容を変更し、または停止することがあります。",
    ],
  },
  {
    title: "6. 免責",
    body: [
      "当サイトは、投稿内容の正確性、有用性、完全性、特定目的適合性を保証しません。",
      "当サイトの利用または利用不能により生じた損害について、当サイト運営者は故意または重過失がある場合を除き責任を負いません。",
    ],
  },
  {
    title: "7. 知的財産権",
    body: [
      "ユーザーが投稿した内容の権利は原則として投稿者に帰属します。ただし、当サイトはサービス運営、表示、保存、改善、紹介のために必要な範囲で当該内容を利用できるものとします。",
    ],
  },
  {
    title: "8. 退会および利用停止",
    body: [
      "ユーザーは、当サイト所定の方法により退会できます。また、規約違反その他運営上必要がある場合、当サイトはアカウント停止または削除を行うことがあります。",
    ],
  },
  {
    title: "9. 規約の変更",
    body: ["当サイトは、必要に応じて本規約を変更できます。変更後の規約は、当サイト上に掲載した時点から効力を生じます。"],
  },
] as const;

export default function TermsPage() {
  return (
    <PageContainer as="article" className="max-w-3xl space-y-8">
      <header className="space-y-2">
        <h1 className="text-3xl font-bold">利用規約</h1>
        <p className="text-sm text-muted-foreground">最終更新日: 2026-03-12</p>
      </header>

      <p className="text-sm leading-7 text-muted-foreground">
        本規約は、当サイトの利用条件を定めるものです。ユーザーは当サイトを利用することにより、本規約に同意したものとみなされます。
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
