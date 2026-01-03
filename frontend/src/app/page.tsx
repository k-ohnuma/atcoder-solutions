import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { ApiClient } from "@/lib/server/apiClient";
import { serverConfig } from "@/shared/config/backend";

const apiClient = new ApiClient(serverConfig.appConfig.apiBaseEndpoint);

export default async function Home() {
  const contestGroupCollection = await apiClient.getContestGroupByContestSeries("ABC");
  const list = [...contestGroupCollection.entries()];

  return (
    <Table>
      <TableBody>
        {list.map(([contestId, problems]) => (
          <TableRow key={contestId}>
            <TableCell className="whitespace-nowrap font-medium">{contestId}</TableCell>

            {problems.map((problem) => (
              <TableCell key={problem.id}>{`${problem.problemIndex}. ${problem.title}`}</TableCell>
            ))}
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
