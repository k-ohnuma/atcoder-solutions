use anyhow::Result;
use domain::model::problem::{ContestSeries, Problem};
use domain::ports::repository::problem::ProblemRepository;
use infrastructure::{database::ConnectionPool, ports::repository::problem::ProblemRepositoryImpl};
use sqlx::PgPool;

#[cfg(test)]
pub async fn seed_contest_series(pool: &PgPool) -> Result<()> {
    for code in ["ABC", "ARC", "AGC", "AHC", "OTHER"] {
        sqlx::query!(
            r#"INSERT INTO contest_series (code)
               VALUES ($1) ON CONFLICT (code) DO NOTHING"#,
            code
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

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

    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contests")
        .fetch_one(&pool)
        .await?;

    assert!(count == 1);

    let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM problems")
        .fetch_one(&pool)
        .await?;
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
    let (title,): (String,) = sqlx::query_as("SELECT title FROM problems WHERE id = 'abc300_a'")
        .fetch_one(&pool)
        .await?;
    assert_eq!(title, "A - Example");

    let updated = Problem {
        title: "A - Example modified".into(),
        ..p1
    };
    repo.create_records(vec![updated]).await?;
    let (title,): (String,) = sqlx::query_as("SELECT title FROM problems WHERE id = 'abc300_a'")
        .fetch_one(&pool)
        .await?;
    assert_eq!(title, "A - Example modified");
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn get_problems_by_contest_series(pool: PgPool) -> Result<()> {
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
    let p3 = Problem {
        id: "arc001_b".into(),
        contest_code: "arc001".into(),
        problem_index: "b".into(),
        title: "B - Example2".into(),
    };
    let p4 = Problem {
        id: "abc001_c".into(),
        contest_code: "abc001".into(),
        problem_index: "c".into(),
        title: "C - 001".into(),
    };

    repo.create_records(vec![p1, p2, p3, p4]).await?;

    let abcs = repo
        .get_problems_by_contest_series(ContestSeries::ABC)
        .await?;
    let arcs = repo
        .get_problems_by_contest_series(ContestSeries::ARC)
        .await?;
    assert!(abcs.len() == 3);
    assert!(arcs.len() == 1);

    println!("{:?}", abcs);

    let ids: Vec<&str> = abcs.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(ids, vec!["abc300_a", "abc300_b", "abc001_c"]);

    assert_eq!(arcs[0].id, "arc001_b");
    Ok(())
}
