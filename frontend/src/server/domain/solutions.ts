import { SolutionDetail } from "@/shared/model/solution";
import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";
import { Resp } from "../response";

export interface SolutionRepository {
  create(solution: Solution, token: string): Promise<Resp<SolutionResponse>>;
  getBySolutionId(solutionId: string): Promise<Resp<SolutionDetail>>;
}
