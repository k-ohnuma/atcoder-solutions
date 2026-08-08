use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::{repository::RepositoryError, user::CreateUserRepositoryError},
    model::user::{Role, User, id::UserId, name::UserName},
    ports::{
        external::auth::{
            AuthProviderError, CreateIdentityError, CreatedIdentity, IdentityProviderPort,
        },
        repository::user::UserRepository,
    },
};

fn user_id(value: &str) -> UserId {
    UserId::try_new(value.to_string()).expect("valid user ID")
}

fn user_name(value: &str) -> UserName {
    UserName::try_new(value.to_string()).expect("valid user name")
}
use usecase::{
    error::UserError,
    model::user::create::CreateUserInput,
    user::{create_user::CreateUserUsecase, delete_me::DeleteMeUsecase},
};

struct DummyUserRepository {
    calls: Mutex<Vec<User>>,
    deleted_uids: Mutex<Vec<String>>,
}

struct DummyIdentityProvider {
    deleted_uids: Mutex<Vec<String>>,
}

#[async_trait]
impl IdentityProviderPort for DummyIdentityProvider {
    async fn create_user(
        &self,
        email: &str,
        _password: &str,
    ) -> Result<CreatedIdentity, CreateIdentityError> {
        if email == "existing-email@example.com" {
            return Err(CreateIdentityError::EmailAlreadyExists);
        }
        if email == "conflict@example.com" {
            return Ok(CreatedIdentity {
                uid: user_id("conflict"),
            });
        }
        if email == "repository-failure@example.com" {
            return Ok(CreatedIdentity {
                uid: user_id("repository-failure"),
            });
        }
        Ok(CreatedIdentity {
            uid: user_id("valid"),
        })
    }

    async fn delete_user(&self, uid: &UserId) -> Result<(), AuthProviderError> {
        self.deleted_uids
            .lock()
            .unwrap()
            .push(uid.as_str().to_string());
        Ok(())
    }
}

fn dummy_identity_provider() -> Arc<DummyIdentityProvider> {
    Arc::new(DummyIdentityProvider {
        deleted_uids: Mutex::new(vec![]),
    })
}

#[async_trait]
impl UserRepository for DummyUserRepository {
    async fn create_user(&self, user: User) -> Result<(), CreateUserRepositoryError> {
        let id = user.id.as_str();
        if id == "conflict" {
            return Err(CreateUserRepositoryError::UserNameAlreadyExists);
        }
        if id == "repository-failure" {
            return Err(CreateUserRepositoryError::Repository(
                RepositoryError::Unexpected("users primary key already exists".to_string()),
            ));
        }
        self.calls.lock().unwrap().push(user);
        Ok(())
    }
    async fn find_by_uid(&self, uid: &UserId) -> Result<Option<User>, RepositoryError> {
        if uid.as_str() == "valid id" {
            return Ok(Some(User {
                id: uid.clone(),
                role: Role::default(),
                user_name: user_name("valid_user"),
            }));
        }
        Ok(None)
    }

    async fn user_name_exists(&self, _user_name: &UserName) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn delete_by_uid(&self, uid: &UserId) -> Result<bool, RepositoryError> {
        self.deleted_uids
            .lock()
            .unwrap()
            .push(uid.as_str().to_string());
        Ok(true)
    }

    async fn revoke_tokens_by_uid(&self, _uid: &UserId) -> Result<bool, RepositoryError> {
        Ok(true)
    }

    async fn is_token_revoked(
        &self,
        _uid: &UserId,
        _issued_at: i64,
    ) -> Result<bool, RepositoryError> {
        Ok(false)
    }
}

#[tokio::test]
async fn usecase_create_user_ok() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo.to_owned(), identity_provider);

    let input = CreateUserInput::try_new(
        "user_name".into(),
        "user@example.com".into(),
        "Password1".into(),
    )?;

    let t = uc.run(input).await;
    assert!(t.is_ok());
    let output = t.unwrap();
    assert_eq!(output.user_name.as_str(), "user_name");

    let calls = repo.calls.lock().unwrap();
    assert!(calls.len() == 1);

    let user = &calls[0];
    let m = &user.role;

    assert!(matches!(m, Role::User));
    assert_eq!(user.user_name.as_str(), "user_name");
    assert_eq!(user.id.as_str(), "valid");

    Ok(())
}

#[tokio::test]
async fn usecase_create_user_returns_user_name_conflict() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo.to_owned(), identity_provider.to_owned());

    let input = CreateUserInput::try_new(
        "user_name".into(),
        "conflict@example.com".into(),
        "Password1".into(),
    )?;

    let error = uc.run(input).await.expect_err("should conflict");

    assert!(matches!(error, UserError::UserNameAlreadyExists(_)));
    assert_eq!(
        identity_provider.deleted_uids.lock().unwrap().as_slice(),
        ["conflict"]
    );
    //
    Ok(())
}

#[tokio::test]
async fn usecase_create_user_returns_email_conflict() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo.to_owned(), identity_provider.to_owned());

    let input = CreateUserInput::try_new(
        "user_name".into(),
        "existing-email@example.com".into(),
        "Password1".into(),
    )?;

    let error = uc.run(input).await.expect_err("should conflict");

    assert!(matches!(error, UserError::EmailAlreadyExists(_)));
    assert!(repo.calls.lock().unwrap().is_empty());
    assert!(identity_provider.deleted_uids.lock().unwrap().is_empty());

    Ok(())
}

#[tokio::test]
async fn usecase_create_user_maps_repository_failure_to_internal_error() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo, identity_provider.to_owned());

    let input = CreateUserInput::try_new(
        "user_name".into(),
        "repository-failure@example.com".into(),
        "Password1".into(),
    )?;

    let error = uc.run(input).await.expect_err("should fail");

    assert!(matches!(error, UserError::Internal(_)));
    assert_eq!(
        identity_provider.deleted_uids.lock().unwrap().as_slice(),
        ["repository-failure"]
    );

    Ok(())
}

#[tokio::test]
async fn usecase_create_user_bad_request_when_user_name_blank() -> Result<()> {
    let err = CreateUserInput::try_new("   ".into(), "user@example.com".into(), "Password1".into())
        .err()
        .expect("should be bad request");
    assert!(matches!(err, UserError::BadRequest(_)));

    Ok(())
}

#[tokio::test]
async fn usecase_create_user_bad_request_when_user_name_contains_non_ascii() -> Result<()> {
    let err = CreateUserInput::try_new(
        "日本語name".into(),
        "user@example.com".into(),
        "Password1".into(),
    )
    .err()
    .expect("should be bad request");
    assert!(matches!(err, UserError::BadRequest(_)));

    Ok(())
}

#[tokio::test]
async fn usecase_create_user_bad_request_when_password_is_weak() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo, identity_provider);

    let input = CreateUserInput::try_new(
        "user_name".into(),
        "user@example.com".into(),
        "password".into(),
    )?;

    let err = uc.run(input).await.expect_err("should be bad request");
    assert!(matches!(err, UserError::BadRequest(_)));

    Ok(())
}

#[tokio::test]
async fn usecase_delete_me_deletes_identity_provider_and_user_row() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = DeleteMeUsecase::new(repo.to_owned(), identity_provider.to_owned());

    let output = uc.run(user_id("valid"), 1_000, 1_100).await?;

    assert_eq!(output.id.as_str(), "valid");
    assert_eq!(
        identity_provider.deleted_uids.lock().unwrap().as_slice(),
        ["valid"]
    );
    assert_eq!(repo.deleted_uids.lock().unwrap().as_slice(), ["valid"]);

    Ok(())
}

#[tokio::test]
async fn usecase_delete_me_requires_recent_login() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = DeleteMeUsecase::new(repo.to_owned(), identity_provider.to_owned());

    let err = uc
        .run(user_id("valid"), 1_000, 1_301)
        .await
        .expect_err("should require recent login");

    assert!(matches!(err, UserError::RequiresRecentLogin(_)));
    assert!(identity_provider.deleted_uids.lock().unwrap().is_empty());
    assert!(repo.deleted_uids.lock().unwrap().is_empty());

    Ok(())
}
