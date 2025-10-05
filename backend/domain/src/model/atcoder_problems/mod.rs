use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ApiProblem {
    pub id: String,
    pub contest_id: String,
    pub problem_index: String,
    pub name: String,
}
