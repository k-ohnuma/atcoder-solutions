import { Badge } from "@/components/ui/badge";
import type { DifficultyBand } from "@/shared/model/problem";

const difficultyBandClass: Record<DifficultyBand, string> = {
  gray: "border-transparent bg-[#808080] text-white",
  brown: "border-transparent bg-[#804000] text-white",
  green: "border-transparent bg-[#008000] text-white",
  cyan: "border-transparent bg-[#00C0C0] text-white",
  blue: "border-transparent bg-[#0000FF] text-white",
  yellow: "border-transparent bg-[#C0C000] text-white",
  orange: "border-transparent bg-[#FF8000] text-white",
  red: "border-transparent bg-[#FF0000] text-white",
};

function difficultyBadgeClass(difficultyBand?: DifficultyBand | null): string {
  if (!difficultyBand) {
    return "";
  }
  return difficultyBandClass[difficultyBand];
}

const compactBadgeClass = "mb-1 px-1.5 py-0 text-[12px] leading-tight";

export function DifficultyBadge({
  problemIndex,
  difficulty,
  difficultyBand,
  label = problemIndex,
}: {
  problemIndex: string;
  difficulty?: number | null;
  difficultyBand?: DifficultyBand | null;
  label?: string | number;
}) {
  return (
    <Badge
      variant="outline"
      className={`${compactBadgeClass} ${difficultyBadgeClass(difficultyBand)}`}
      title={`Difficulty: ${difficulty ?? "N/A"}`}
    >
      {label}
    </Badge>
  );
}
