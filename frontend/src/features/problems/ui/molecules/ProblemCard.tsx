import Link from "next/link";
import { Problem } from "@/shared/model/problem";
import { DifficultyBadge } from "../atoms/DifficultyBadge";

export function ProblemCard({ problem }: { problem: Problem }) {
  return (
    <Link
      href={`/problems/${problem.id}`}
      className="block rounded-md border bg-background px-3 py-2 transition-colors hover:bg-accent"
    >
      <DifficultyBadge problemIndex={problem.problemIndex} difficulty={problem.difficulty} />
      <p className="text-sm font-medium">{problem.title}</p>
    </Link>
  );
}
