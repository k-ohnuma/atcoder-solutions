import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Problem } from "@/shared/model/problem";
import { ProblemCard } from "../molecules/ProblemCard";

export function ContestProblemSections({
  contests,
}: {
  contests: Array<readonly [string, Problem[]]>;
}) {
  return (
    <section className="space-y-3">
      {contests.map(([contestId, problems]) => (
        <Card key={contestId}>
          <CardHeader className="pb-3">
            <CardTitle className="text-base">{contestId}</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-2 pt-0 sm:grid-cols-2 lg:grid-cols-3">
            {problems.map((problem) => (
              <ProblemCard key={problem.id} problem={problem} />
            ))}
          </CardContent>
        </Card>
      ))}
    </section>
  );
}
