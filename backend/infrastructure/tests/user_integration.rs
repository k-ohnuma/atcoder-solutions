use anyhow::Result;
use domain::{
    error::repository::RepositoryError,
    model::user::{Role, User},
    ports::repository::user::UserRepository,
};
use infrastructure::{database::ConnectionPool, ports::repository::user::UserRepositoryImpl};
use sqlx::PgPool;

pub async fn seed_roles(pool: &PgPool) -> Result<()> {
    for role in ["admin", "user"] {
        sqlx::query!(
            r#"
            INSERT INTO roles (name)
            VALUES ($1) ON CONFLICT DO NOTHING
            "#,
            role
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

fn make_user(id: &str, name: &str) -> User {
    User {
        id: id.into(),
        user_name: name.into(),
        role: Role::default(),
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn user_records(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);
    let user = make_user("id", "name");
    let res = repo.create_user(user).await;
    assert!(res.is_ok());

    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE user_name = $1", "name")
        .fetch_one(&pool)
        .await?;
    assert!(count.is_some());
    let count = count.unwrap();
    assert!(count == 1);

    let user = make_user("id2", "name2");
    let res = repo.create_user(user).await;
    assert!(res.is_ok());

    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    assert!(count == 2);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn create_user_conflict_on_username_returns_conflict_error(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);

    let u1 = make_user("id-a", "alice");
    repo.create_user(u1).await.unwrap();

    let u2 = make_user("id-b", "alice");
    let err = repo.create_user(u2).await.expect_err("should conflict");

    match err {
        RepositoryError::UniqueViolation(msg) => {
            assert!(
                msg.contains("alice"),
                "message should mention user_name, got: {msg}"
            );
        }
        other => panic!("expected Conflict, got {:?}", other),
    }

    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE user_name = $1", "alice")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    assert_eq!(count, 1);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn create_user_same_id_is_db_error_not_conflict(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);

    let u1 = make_user("same-id", "bob");
    repo.create_user(u1).await.unwrap();

    let u2 = make_user("same-id", "bob2");
    let err = repo.create_user(u2).await.expect_err("pk(id) should error");

    match err {
        RepositoryError::UniqueViolation(_msg) => {}
        other => panic!("expected unique value violation, got {:?}", other),
    }
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE id = $1", "same-id")
        .fetch_one(&pool)
        .await
        .unwrap()
        .unwrap_or(0);
    assert_eq!(count, 1);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn find_by_uid(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);

    let u1 = make_user("id1", "bob1");
    let u2 = make_user("id2", "bob2");
    let u3 = make_user("id3", "bob3");
    for user in [u1, u2, u3] {
        repo.create_user(user).await?;
    }

    let user = repo.find_by_uid("id1").await?;
    assert_eq!(user.user_name, "bob1");

    let user = repo.find_by_uid("id3").await?;
    assert_eq!(user.user_name, "bob3");

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn find_by_uid_no_record(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);

    let u1 = make_user("id1", "bob1");
    let u2 = make_user("id2", "bob2");
    let u3 = make_user("id3", "bob3");
    for user in [u1, u2, u3] {
        repo.create_user(user).await?;
    }

    let err = repo.find_by_uid("id999").await.expect_err("should error");
    let mat = matches!(err, RepositoryError::NotFound(_));
    assert!(mat, "should be not found error");

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn revoke_tokens_and_check_revoked_status(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);

    let uid = "revoke-target";
    repo.create_user(make_user(uid, "revoker")).await?;

    let before_revoke = repo.is_token_revoked(uid, 0).await?;
    assert!(!before_revoke);

    repo.revoke_tokens_by_uid(uid).await?;

    let revoked_epoch = sqlx::query_scalar!(
        r#"
        SELECT EXTRACT(EPOCH FROM token_revoked_before)::BIGINT AS "epoch"
        FROM users
        WHERE id = $1
        "#,
        uid
    )
    .fetch_one(&pool)
    .await?
    .unwrap_or(0);
    assert!(revoked_epoch > 0);

    let should_be_revoked = repo.is_token_revoked(uid, revoked_epoch - 1).await?;
    assert!(should_be_revoked);

    let should_not_be_revoked_same_second = repo.is_token_revoked(uid, revoked_epoch).await?;
    assert!(!should_not_be_revoked_same_second);

    let should_not_be_revoked = repo.is_token_revoked(uid, revoked_epoch + 1).await?;
    assert!(!should_not_be_revoked);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn missing_user_is_treated_as_revoked(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);

    let revoked = repo.is_token_revoked("missing-user", 0).await?;
    assert!(revoked);

    Ok(())
}
