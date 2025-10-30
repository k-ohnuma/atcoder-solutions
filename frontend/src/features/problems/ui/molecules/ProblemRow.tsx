import { Problem } from "@/shared/model/problem";
import Link from "next/link";
import { MetaBadge, ProblemIndex } from "../atoms";
import { css } from "styled-system/css";

export function ProblemRow({ p }: { p: Problem }) {
  const href = `/problems/${p.id}`
  return (
    <li className={css({py: '2', px: '4'})}>
      <Link href={href} className={css({display: 'flex', alignItems: 'center', gap: '3', _hover: {textDecoration: 'underline'}})}>
        <ProblemIndex id={p.id} />
        <span className={css({fontSize: 'sm'})}>{p.title}</span>
        <MetaBadge title="解説数">{0}</MetaBadge>
      </Link>
    </li>
  );
}
