use anyhow::Result;
use domain::{
    model::user::{Role, User},
    ports::repository::user::UserRepository,
};
use infrastructure::{database::ConnectionPool, ports::repository::user::UserRepositoryImpl};
use shared::error::repository::RepositoryError;
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

fn make_user(id: &str, name: &str, color: &str) -> User {
    User {
        id: id.into(),
        color: color.into(),
        user_name: name.into(),
        role: Role::default(),
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn user_records(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);
    let user = make_user("id", "name", "#000000");
    let res = repo.create_user(user).await;
    assert!(res.is_ok());

    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE user_name = $1", "name")
        .fetch_one(&pool)
        .await?;
    assert!(count.is_some());
    let count = count.unwrap();
    assert!(count == 1);

    let user = make_user("id2", "name2", "#000000");
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

    let u1 = make_user("id-a", "alice", "#000000");
    repo.create_user(u1).await.unwrap();

    let u2 = make_user("id-b", "alice", "#000000");
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

    let u1 = make_user("same-id", "bob", "#000000");
    repo.create_user(u1).await.unwrap();

    let u2 = make_user("same-id", "bob2", "#ffffff");
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

    let u1 = make_user("id1", "bob1", "#000000");
    let u2 = make_user("id2", "bob2", "#ffffff");
    let u3 = make_user("id3", "bob3", "#ffffff");
    for user in [u1, u2, u3] {
        repo.create_user(user).await?;
    }

    let user = repo.find_by_uid("id1").await?;
    assert_eq!(user.user_name, "bob1");
    assert_eq!(user.color, "#000000");

    let user = repo.find_by_uid("id3").await?;
    assert_eq!(user.user_name, "bob3");
    assert_eq!(user.color, "#ffffff");

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn find_by_uid_no_record(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(conn);

    let u1 = make_user("id1", "bob1", "#000000");
    let u2 = make_user("id2", "bob2", "#ffffff");
    let u3 = make_user("id3", "bob3", "#ffffff");
    for user in [u1, u2, u3] {
        repo.create_user(user).await?;
    }

    let err = repo.find_by_uid("id999").await.expect_err("should error");
    let mat = matches!(err, RepositoryError::NotFound(_));
    assert!(mat, "should be not found error");

    Ok(())
}
