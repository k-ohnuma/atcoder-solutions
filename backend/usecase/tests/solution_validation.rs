use anyhow::Result;
use domain::model::user::id::UserId;
use usecase::model::solution::{
    create_comment::CreateCommentInput, update_comment::UpdateCommentInput,
};
use uuid::Uuid;

fn user_id(value: &str) -> UserId {
    UserId::try_new(value.to_string()).expect("valid user ID")
}

#[test]
fn comment_body_length_boundaries() -> Result<()> {
    assert!(
        CreateCommentInput::try_new(user_id("uid"), Uuid::now_v7().into(), "a".repeat(2_000),)
            .is_ok()
    );
    assert!(
        CreateCommentInput::try_new(user_id("uid"), Uuid::now_v7().into(), "a".repeat(2_001),)
            .is_err()
    );
    assert!(
        UpdateCommentInput::try_new(user_id("uid"), Uuid::now_v7().into(), "a".repeat(2_000),)
            .is_ok()
    );
    assert!(
        UpdateCommentInput::try_new(user_id("uid"), Uuid::now_v7().into(), "a".repeat(2_001),)
            .is_err()
    );
    Ok(())
}
