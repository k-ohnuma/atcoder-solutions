import { css } from "styled-system/css";
import { Problem } from "@/server/domain/problems";
import { ProblemRow } from "../molecules/ProblemRow";

export function ProblemsList({ problems }: { problems: Problem[] }) {
  return (
    <ul
      className={css({
        display: "flex",
        flexDirection: "column",
        divideY: "1px",
      })}
    >
      {problems.map((p) => (
        <ProblemRow key={p.id} p={p} />
      ))}
    </ul>
  );
}
