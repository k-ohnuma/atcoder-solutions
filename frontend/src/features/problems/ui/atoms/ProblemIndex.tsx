import { css } from "styled-system/css";

export function ProblemIndex({ id }: { id: string }) {
  return (
    // <span className="w-20 tabular-nums text-sm opacity-60">
    <span className={css({
      w: 20,
      fontSize: 'sm',
    })}>
      {id.toUpperCase()}
    </span>
  );
}
