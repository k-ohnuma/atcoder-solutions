import { Solution, SolutionResponse } from "@/shared/model/solutionCreate";
import { Resp } from "../response";

export interface SolutionRepository {
  create(solution: Solution, token: string): Promise<Resp<SolutionResponse>>;
}
