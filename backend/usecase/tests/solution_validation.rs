use anyhow::Result;
use usecase::model::solution::{
    create::CreateSolutionInput, create_comment::CreateCommentInput, update::UpdateSolutionInput,
    update_comment::UpdateCommentInput,
};
use uuid::Uuid;
use validator::Validate;

#[test]
fn create_solution_title_length_boundaries() -> Result<()> {
    let ok = CreateSolutionInput {
        user_id: "uid".to_string(),
        title: "a".repeat(120),
        problem_id: "abc100_a".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
        tags: vec![],
    };
    assert!(ok.validate().is_ok());

    let ng = CreateSolutionInput {
        title: "a".repeat(121),
        ..ok
    };
    assert!(ng.validate().is_err());
    Ok(())
}

#[test]
fn create_solution_body_length_boundaries() -> Result<()> {
    let ok = CreateSolutionInput {
        user_id: "uid".to_string(),
        title: "title".to_string(),
        problem_id: "abc100_a".to_string(),
        body_md: "a".repeat(20_000),
        submit_url: String::new(),
        tags: vec![],
    };
    assert!(ok.validate().is_ok());

    let ng = CreateSolutionInput {
        body_md: "a".repeat(20_001),
        ..ok
    };
    assert!(ng.validate().is_err());
    Ok(())
}

#[test]
fn create_solution_submit_url_allows_blank_and_http_https_only() -> Result<()> {
    let blank = CreateSolutionInput {
        user_id: "uid".to_string(),
        title: "title".to_string(),
        problem_id: "abc100_a".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
        tags: vec![],
    };
    assert!(blank.validate().is_ok());

    let https = CreateSolutionInput {
        submit_url: "https://example.com".to_string(),
        ..blank
    };
    assert!(https.validate().is_ok());

    let ftp = CreateSolutionInput {
        submit_url: "ftp://example.com".to_string(),
        ..https
    };
    assert!(ftp.validate().is_err());
    Ok(())
}

#[test]
fn create_solution_tags_allows_zero_and_rejects_over_six() -> Result<()> {
    let zero = CreateSolutionInput {
        user_id: "uid".to_string(),
        title: "title".to_string(),
        problem_id: "abc100_a".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
        tags: vec![],
    };
    assert!(zero.validate().is_ok());

    let seven = CreateSolutionInput {
        tags: vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
        ],
        ..zero
    };
    assert!(seven.validate().is_err());
    Ok(())
}

#[test]
fn update_solution_title_and_body_boundaries() -> Result<()> {
    let ok = UpdateSolutionInput {
        user_id: "uid".to_string(),
        solution_id: Uuid::now_v7(),
        title: "a".repeat(120),
        body_md: "a".repeat(20_000),
        submit_url: String::new(),
        tags: vec![],
    };
    assert!(ok.validate().is_ok());

    let ng_title = UpdateSolutionInput {
        title: "a".repeat(121),
        ..ok
    };
    assert!(ng_title.validate().is_err());
    Ok(())
}

#[test]
fn comment_body_length_boundaries() -> Result<()> {
    let ok_create = CreateCommentInput {
        user_id: "uid".to_string(),
        solution_id: Uuid::now_v7(),
        body_md: "a".repeat(2000),
    };
    assert!(ok_create.validate().is_ok());

    let ng_create = CreateCommentInput {
        body_md: "a".repeat(2001),
        ..ok_create
    };
    assert!(ng_create.validate().is_err());

    let ok_update = UpdateCommentInput {
        user_id: "uid".to_string(),
        comment_id: Uuid::now_v7(),
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
