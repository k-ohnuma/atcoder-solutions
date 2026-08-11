use domain::model::{
    solution::{comment_body::CommentBody, comment_id::CommentId},
    user::id::UserId,
};

pub struct UpdateCommentInput {
    pub user_id: UserId,
    pub comment_id: CommentId,
    pub body: CommentBody,
}

impl UpdateCommentInput {
    pub fn try_new(
        user_id: UserId,
        comment_id: CommentId,
        body: String,
    ) -> Result<Self, crate::error::SolutionError> {
        Ok(Self {
            user_id,
            comment_id,
            body: CommentBody::try_new(body)?,
        })
    }
}
