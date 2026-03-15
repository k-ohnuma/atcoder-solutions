import { Heart } from "lucide-react";
import Link from "next/link";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";

const dateTimeFormatter = new Intl.DateTimeFormat("ja-JP", {
  timeZone: "Asia/Tokyo",
  year: "numeric",
  month: "2-digit",
  day: "2-digit",
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  hour12: false,
});

type SolutionSummaryCardProps = {
  href: string;
  title: string;
  votesCount: number;
  createdAt: string;
  problemId?: string;
  problemTitle?: string;
  userName?: string;
  footerLabel?: string;
};

export function SolutionSummaryCard({
  href,
  title,
  votesCount,
  createdAt,
  problemId,
  problemTitle,
  userName,
  footerLabel,
}: SolutionSummaryCardProps) {
  return (
    <Link href={href} className="block">
      <Card className="transition-colors hover:bg-accent">
        <CardContent className="p-4">
          <div className="mb-3 flex items-start justify-between gap-3">
            <h2 className="line-clamp-2 text-lg font-semibold leading-snug">{title}</h2>
            <Badge variant="outline" className="shrink-0 gap-1 rounded-md px-2 py-1 text-sm">
              <Heart className="size-4" />
              {votesCount}
            </Badge>
          </div>

          {(problemId || problemTitle || userName) && (
            <div className="mb-2 flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
              {problemId && <Badge variant="secondary">{problemId}</Badge>}
              {problemTitle && <span>{problemTitle}</span>}
              {userName && <span>投稿者: {userName}</span>}
            </div>
          )}

          <div className="text-xs text-muted-foreground">{dateTimeFormatter.format(new Date(createdAt))}</div>

          {footerLabel && <p className="mt-4 text-sm font-medium">{footerLabel}</p>}
        </CardContent>
      </Card>
    </Link>
  );
}
