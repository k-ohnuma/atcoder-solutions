import { Problem } from "@/server/domain/problems";
import { ProblemRow } from "../molecules/ProblemRow";

export function ProblemsList({ problems }: { problems: Problem[] }) {
  return (
    <ul className="flex flex-col divide-y">
      {problems.map((p) => (
        <ProblemRow key={p.id} p={p} />
      ))}
    </ul>
  );
}
