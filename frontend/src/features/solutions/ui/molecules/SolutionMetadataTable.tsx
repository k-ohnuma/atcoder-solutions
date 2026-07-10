import Link from "next/link";
import { Badge } from "@/components/ui/badge";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { formatDateTime } from "@/shared/lib/date";

type SolutionMetadataTableProps = {
  userName: string;
  createdAt: string;
  atcoderProblemUrl: string;
  submitUrl: string;
  tags: string[];
};

export function SolutionMetadataTable({ userName, createdAt, atcoderProblemUrl, submitUrl, tags }: SolutionMetadataTableProps) {
  return (
    <Table>
      <TableBody>
        <TableRow>
          <TableCell className="w-28 font-medium">投稿者</TableCell>
          <TableCell>
            <Link href={`/users/${encodeURIComponent(userName)}/solutions`} className="hover:underline">
              {userName}
            </Link>
          </TableCell>
        </TableRow>
        <TableRow>
          <TableCell className="font-medium">投稿日</TableCell>
          <TableCell>{formatDateTime(createdAt)}</TableCell>
        </TableRow>
        <TableRow>
          <TableCell className="font-medium">問題URL</TableCell>
          <TableCell>
            <Link href={atcoderProblemUrl} target="_blank" rel="noreferrer" className="text-sm break-all hover:underline">
              {atcoderProblemUrl}
            </Link>
          </TableCell>
        </TableRow>
        <TableRow>
          <TableCell className="font-medium">提出URL</TableCell>
          <TableCell>
            {submitUrl ? (
              <Link href={submitUrl} target="_blank" rel="noreferrer" className="text-sm break-all hover:underline">
                {submitUrl}
              </Link>
            ) : (
              <span className="text-sm text-muted-foreground">なし</span>
            )}
          </TableCell>
        </TableRow>
        <TableRow>
          <TableCell className="font-medium">タグ</TableCell>
          <TableCell>
            <div className="flex flex-wrap gap-2">
              {tags.length === 0 ? (
                <Badge variant="outline">タグなし</Badge>
              ) : (
                tags.map((tag) => (
                  <Badge key={tag} variant="outline">
                    {tag}
                  </Badge>
                ))
              )}
            </div>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  );
}
