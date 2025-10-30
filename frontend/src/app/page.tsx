import { Table } from "@/components/ui/table";
import { ProblemsTemplate } from "@/features/problems/ui/templates/ProblemsTemplate";
import { ApiClient } from "@/lib/apiClient";
import { serverConfig } from "@/shared/config/backend";

const apiClient = new ApiClient(serverConfig.appConfig.apiBaseEndpoint);
export default async function Home() {
  const contestGroupCollection =
    await apiClient.getContestGroupByContestSeries("OTHER");
  const list = [...contestGroupCollection.entries()];

  // return <ProblemsTemplate problemsMap={contestGroupCollection} />;
  return (
    <Table.Root variant={'outline'}>
      <Table.Body>
        {list.map(([contestId, problems]) => {
          return (
            <Table.Row key={contestId}>
              <Table.Cell>{contestId}</Table.Cell>
              {problems.map((problem) => {
                return <Table.Cell key={problem.id}>{`${problem.problemIndex}. ${problem.title}`}</Table.Cell>;
              })}
            </Table.Row>
          );
        })}
      </Table.Body>
    </Table.Root>
  );
}
