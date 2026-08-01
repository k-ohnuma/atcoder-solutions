use uuid::Uuid;

pub mod content;
pub mod tags;

use content::{SolutionBody, SolutionTitle, SubmitUrl};

pub struct Solution {
    id: Uuid,
    problem_id: String,
    user_id: String,
    title: SolutionTitle,
    body: SolutionBody,
    submit_url: SubmitUrl,
}

impl Solution {
    pub fn new(
        id: Uuid,
        problem_id: String,
        user_id: String,
        title: SolutionTitle,
        body: SolutionBody,
        submit_url: SubmitUrl,
    ) -> Self {
        Self {
            id,
            problem_id,
            user_id,
            title,
            body,
            submit_url,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn problem_id(&self) -> &str {
        &self.problem_id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn title(&self) -> &SolutionTitle {
        &self.title
    }

    pub fn body(&self) -> &SolutionBody {
        &self.body
    }

    pub fn submit_url(&self) -> &SubmitUrl {
        &self.submit_url
    }
}
