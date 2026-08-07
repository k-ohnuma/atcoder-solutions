pub mod comment_body;
pub mod comment_id;
pub mod content;
pub mod id;
pub mod tags;

use crate::model::problem::id::ProblemId;
use crate::model::user::id::UserId;
use content::{SolutionBody, SolutionTitle, SubmitUrl};
use id::SolutionId;

pub struct Solution {
    id: SolutionId,
    problem_id: ProblemId,
    user_id: UserId,
    title: SolutionTitle,
    body: SolutionBody,
    submit_url: SubmitUrl,
}

impl Solution {
    pub fn new(
        id: SolutionId,
        problem_id: ProblemId,
        user_id: UserId,
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

    pub fn id(&self) -> SolutionId {
        self.id
    }

    pub fn problem_id(&self) -> &ProblemId {
        &self.problem_id
    }

    pub fn user_id(&self) -> &UserId {
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
