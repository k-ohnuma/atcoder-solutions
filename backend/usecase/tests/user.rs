use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::repository::RepositoryError,
    model::user::{Role, User},
    ports::{
        external::auth::{AuthError, CreatedIdentity, IdentityProviderPort},
        repository::user::UserRepository,
    },
};
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
    ) -> Result<CreatedIdentity, AuthError> {
        if email == "conflict@example.com" {
            return Ok(CreatedIdentity {
                uid: "conflict".into(),
            });
        }
        Ok(CreatedIdentity {
            uid: "valid".into(),
        })
    }

    async fn delete_user(&self, uid: &str) -> Result<(), AuthError> {
        self.deleted_uids.lock().unwrap().push(uid.to_string());
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
    async fn create_user(&self, user: User) -> Result<(), RepositoryError> {
        let id = user.id.as_str();
        if id == "conflict" {
            return Err(RepositoryError::Conflict("already exist".into()));
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

    async fn user_name_exists(&self, _user_name: &str) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn delete_by_uid(&self, uid: &str) -> Result<(), RepositoryError> {
        self.deleted_uids.lock().unwrap().push(uid.to_string());
        Ok(())
    }

    async fn revoke_tokens_by_uid(&self, _uid: &str) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn is_token_revoked(&self, _uid: &str, _issued_at: i64) -> Result<bool, RepositoryError> {
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

    let input = CreateUserInput {
        user_name: "user_name".into(),
        email: "user@example.com".into(),
        password: "Password1".into(),
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
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo.to_owned(), identity_provider.to_owned());

    let input = CreateUserInput {
        user_name: "user_name".into(),
        email: "conflict@example.com".into(),
        password: "Password1".into(),
    };

    let t = uc.run(input).await.expect_err("conf");

    matches!(t, UserError::Conflict(_));
    assert_eq!(
        identity_provider.deleted_uids.lock().unwrap().as_slice(),
        ["conflict"]
    );
    //
    Ok(())
}

#[tokio::test]
async fn usecase_create_user_bad_request_when_user_name_blank() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo, identity_provider);

    let input = CreateUserInput {
        user_name: "   ".into(),
        email: "user@example.com".into(),
        password: "Password1".into(),
    };

    let err = uc.run(input).await.expect_err("should be bad request");
    assert!(matches!(err, UserError::BadRequest(_)));

    Ok(())
}

#[tokio::test]
async fn usecase_create_user_bad_request_when_user_name_contains_non_ascii() -> Result<()> {
    let repo = Arc::new(DummyUserRepository {
        calls: Mutex::new(vec![]),
        deleted_uids: Mutex::new(vec![]),
    });
    let identity_provider = dummy_identity_provider();
    let uc = CreateUserUsecase::new(repo, identity_provider);

    let input = CreateUserInput {
        user_name: "日本語name".into(),
        email: "user@example.com".into(),
        password: "Password1".into(),
    };

    let err = uc.run(input).await.expect_err("should be bad request");
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

    let input = CreateUserInput {
        user_name: "user_name".into(),
        email: "user@example.com".into(),
        password: "password".into(),
    };

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

    let output = uc.run("valid".into(), 1_000, 1_100).await?;

    assert_eq!(output.id, "valid");
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
        .run("valid".into(), 1_000, 1_301)
        .await
        .expect_err("should require recent login");

    assert!(matches!(err, UserError::RequiresRecentLogin(_)));
    assert!(identity_provider.deleted_uids.lock().unwrap().is_empty());
    assert!(repo.deleted_uids.lock().unwrap().is_empty());

    Ok(())
}
