use domain::model::solution::Solution;
use uuid::Uuid;

pub struct CreateSolutionInput {
    pub user_id: String,
    pub title: String,
    pub problem_id: String,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}

pub fn from_create_solution_input_for_solution(
    uuid: Uuid,
    input: &CreateSolutionInput,
) -> Solution {
    Solution {
        id: uuid,
        title: input.title.to_owned(),
        user_id: input.user_id.to_owned(),
        problem_id: input.problem_id.to_owned(),
        body_md: input.body_md.to_owned(),
        submit_url: input.submit_url.to_owned(),
    }
}
