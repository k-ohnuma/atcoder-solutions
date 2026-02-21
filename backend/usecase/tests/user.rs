use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::repository::RepositoryError,
    model::user::{Role, User},
    ports::repository::user::UserRepository,
};
use usecase::{
    model::user::{UserError, create::CreateUserInput},
    user::create_user::CreateUserUsecase,
};

struct DummyUserRepository {
    calls: Mutex<Vec<User>>,
}

#[async_trait]
impl UserRepository for DummyUserRepository {
    async fn create_user(&self, user: User) -> Result<(), RepositoryError> {
        let id = user.id.as_str();
        if id == "conflict" {
            return Err(RepositoryError::UniqueViolation("already exist".into()));
        }
        self.calls.lock().unwrap().push(user);
        Ok(())
    }
    async fn find_by_uid(&self, uid: &str) -> Result<User, RepositoryError> {
        if uid == "valid id" {
            return Ok(User {
                id: uid.into(),
                role: Role::default(),
                user_name: "valid user".into(),
            });
        }
        Err(RepositoryError::NotFound("Not found".into()))
    }
}

#[tokio::test]
async fn usecase_create_user_ok() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
    });
    let uc = CreateUserUsecase::new(repo.to_owned());

    let input = CreateUserInput {
        uid: "valid".into(),
        user_name: "user_name".into(),
    };

    let t = uc.run(input).await;
    assert!(t.is_ok());
    let output = t.unwrap();
    assert_eq!(output.user_name, "user_name");

    let calls = repo.calls.lock().unwrap();
    assert!(calls.len() == 1);

    let user = &calls[0];
    let m = &user.role;

    assert!(matches!(m, Role::User));
    assert_eq!(user.user_name, "user_name");
    assert_eq!(user.id, "valid");

    Ok(())
}

#[tokio::test]
async fn usecase_create_user_conflict() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
    });
    let uc = CreateUserUsecase::new(repo.to_owned());

    let input = CreateUserInput {
        uid: "conflict".into(),
        user_name: "user_name".into(),
    };

    let t = uc.run(input).await.expect_err("conf");

    matches!(t, UserError::Conflict(_));
    //
    Ok(())
}

#[tokio::test]
async fn usecase_create_user_bad_request_when_user_name_blank() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
    });
    let uc = CreateUserUsecase::new(repo);

    let input = CreateUserInput {
        uid: "valid".into(),
        user_name: "   ".into(),
    };

    let err = uc.run(input).await.expect_err("should be bad request");
    assert!(matches!(err, UserError::BadRequest(_)));

    Ok(())
}
