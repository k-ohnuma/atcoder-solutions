use anyhow::Result;
use domain::model::user::id::UserId;
use usecase::model::solution::{
    create_comment::CreateCommentInput, update_comment::UpdateCommentInput,
};
use uuid::Uuid;
use validator::Validate;

fn user_id(value: &str) -> UserId {
    UserId::try_new(value.to_string()).expect("valid user ID")
}

#[test]
fn comment_body_length_boundaries() -> Result<()> {
    let ok_create = CreateCommentInput {
        user_id: user_id("uid"),
        solution_id: Uuid::now_v7().into(),
        body_md: "a".repeat(2000),
    };
    assert!(ok_create.validate().is_ok());

    let ng_create = CreateCommentInput {
        body_md: "a".repeat(2001),
        ..ok_create
    };
    assert!(ng_create.validate().is_err());

    let ok_update = UpdateCommentInput {
        user_id: user_id("uid"),
        comment_id: Uuid::now_v7().into(),
        body_md: "a".repeat(2000),
    };
    assert!(ok_update.validate().is_ok());

    let ng_update = UpdateCommentInput {
        body_md: "a".repeat(2001),
        ..ok_update
    };
    assert!(ng_update.validate().is_err());
    Ok(())
}
