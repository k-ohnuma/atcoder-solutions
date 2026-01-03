import Link from "next/link";
import { Problem } from "@/shared/model/problem";
import { MetaBadge, ProblemIndex } from "../atoms";

export function ProblemRow({ p }: { p: Problem }) {
  const href = `/problems/${p.id}`;
  return (
    <li className="px-4 py-2">
      <Link href={href} className="flex items-center gap-3 hover:underline">
        <ProblemIndex id={p.id} />
        <span className="text-sm">{p.title}</span>
        <MetaBadge title="解説数">{0}</MetaBadge>
      </Link>
    </li>
  );
}
