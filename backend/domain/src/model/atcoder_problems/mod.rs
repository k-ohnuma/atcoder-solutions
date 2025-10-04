use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiProblem {
    id: String,
    contest_id: String,
    problem_index: String,
    name: String,
}
