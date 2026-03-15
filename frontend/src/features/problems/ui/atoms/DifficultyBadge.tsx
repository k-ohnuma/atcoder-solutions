import { Badge } from "@/components/ui/badge";

function difficultyBadgeClass(difficulty?: number | null): string {
  if (difficulty === null || difficulty === undefined) {
    return "";
  }
  if (difficulty < 400) {
    return "border-transparent bg-[#808080] text-white";
  }
  if (difficulty < 800) {
    return "border-transparent bg-[#804000] text-white";
  }
  if (difficulty < 1200) {
    return "border-transparent bg-[#008000] text-white";
  }
  if (difficulty < 1600) {
    return "border-transparent bg-[#00C0C0] text-white";
  }
  if (difficulty < 2000) {
    return "border-transparent bg-[#0000FF] text-white";
  }
  if (difficulty < 2400) {
    return "border-transparent bg-[#C0C000] text-white";
  }
  if (difficulty < 2800) {
    return "border-transparent bg-[#FF8000] text-white";
  }
  return "border-transparent bg-[#FF0000] text-white";
}

const compactBadgeClass = "mb-1 px-1.5 py-0 text-[12px] leading-tight";

export function DifficultyBadge({
  problemIndex,
  difficulty,
}: {
  problemIndex: string;
  difficulty?: number | null;
}) {
  return (
    <Badge
      variant="outline"
      className={`${compactBadgeClass} ${difficultyBadgeClass(difficulty)}`}
      title={`Difficulty: ${difficulty ?? "N/A"}`}
    >
      {problemIndex}
    </Badge>
  );
}
