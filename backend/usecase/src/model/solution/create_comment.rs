use domain::model::{
    solution::{comment_body::CommentBody, id::SolutionId},
    user::id::UserId,
};

pub struct CreateCommentInput {
    pub user_id: UserId,
    pub solution_id: SolutionId,
    pub body: CommentBody,
}

impl CreateCommentInput {
    pub fn try_new(
        user_id: UserId,
        solution_id: SolutionId,
        body: String,
    ) -> Result<Self, crate::error::SolutionError> {
        Ok(Self {
            user_id,
            solution_id,
            body: CommentBody::try_new(body)?,
        })
    }
}
