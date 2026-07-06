import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ContestProblemGroup } from "@/features/problems/model/contestProblemGroup";
import { ProblemCard } from "../molecules/ProblemCard";

export function ContestProblemSections({ groups }: { groups: ContestProblemGroup[] }) {
  return (
    <section className="space-y-3">
      {groups.map((group) => (
        <Card key={group.contestId}>
          <CardHeader className="pb-3">
            <CardTitle className="text-base">{group.contestId}</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-2 pt-0 sm:grid-cols-2 lg:grid-cols-3">
            {group.problems.map((problem) => (
              <ProblemCard key={problem.id} problem={problem} />
            ))}
          </CardContent>
        </Card>
      ))}
    </section>
  );
}
