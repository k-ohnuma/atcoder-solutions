use anyhow::Result;
use common::seed_contest_series;
use domain::model::problem::Problem;
use domain::ports::repository::problem::ProblemRepository;
use infrastructure::{database::ConnectionPool, ports::repository::problem::ProblemRepositoryImpl};
use sqlx::PgPool;

pub mod common;

#[sqlx::test(migrations = "./migrations")]
async fn create_records(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = ProblemRepositoryImpl::new(conn);
    let p1 = Problem {
        id: "abc300_a".into(),
        contest_code: "abc300".into(),
        problem_index: "a".into(),
        title: "A - Example".into(),
    };
    let p2 = Problem {
        id: "abc300_b".into(),
        contest_code: "abc300".into(),
        problem_index: "b".into(),
        title: "B - Example".into(),
    };

    repo.create_records(vec![p1, p2]).await?;

    let (count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM contests")
            .fetch_one(&pool)
            .await?;

    assert!(count == 1);

    let (count, ) : (i64,) = sqlx::query_as("SELECT count(*) FROM problems").fetch_one(&pool).await?;
    assert!(count == 2);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn upsert_records(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = ProblemRepositoryImpl::new(conn);
    let p1 = Problem {
        id: "abc300_a".into(),
        contest_code: "abc300".into(),
        problem_index: "a".into(),
        title: "A - Example".into(),
    };
    repo.create_records(vec![p1.to_owned()]).await?;
    let (title, ): (String, ) = sqlx::query_as("SELECT title FROM problems WHERE id = 'abc300_a'").fetch_one(&pool).await?;
    assert_eq!(title, "A - Example");

    let updated = Problem{title: "A - Example modified".into(), ..p1};
    repo.create_records(vec![updated]).await?;
    let (title, ): (String, ) = sqlx::query_as("SELECT title FROM problems WHERE id = 'abc300_a'").fetch_one(&pool).await?;
    assert_eq!(title, "A - Example modified");
    Ok(())
}

