use anyhow::Result;
use domain::{
    model::problem::{
        Problem, contest_code::ContestCode, id::ProblemId, index::ProblemIndex, title::ProblemTitle,
    },
    ports::repository::problem::tx::ProblemTxManager,
};
use infrastructure::{
    database::ConnectionPool, ports::repository::problem::tx::ProblemTransactionManager,
};
use sqlx::PgPool;

fn problem_id(value: &str) -> ProblemId {
    ProblemId::try_new(value.to_string()).expect("valid problem ID")
}

fn contest_code(value: &str) -> ContestCode {
    ContestCode::try_new(value.to_string()).expect("valid contest code")
}

fn problem_index(value: &str) -> ProblemIndex {
    ProblemIndex::try_new(value.to_string()).expect("valid problem index")
}

fn problem_title(value: &str) -> ProblemTitle {
    ProblemTitle::try_new(value.to_string()).expect("valid problem title")
}

async fn seed_contest_series(pool: &PgPool) -> Result<()> {
    for code in ["ABC", "ARC", "AGC", "AHC", "AWC", "OTHER"] {
        sqlx::query!(
            r#"INSERT INTO contest_series (code)
               VALUES ($1) ON CONFLICT DO NOTHING"#,
            code
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

async fn upsert_problem_data(
    pool: &PgPool,
    contests: &[(String, String)],
    problems: &[Problem],
) -> Result<()> {
    let manager = ProblemTransactionManager::new(ConnectionPool::new(pool.clone()));
    let mut uow = manager.begin().await?;
    uow.problems().upsert_contests(contests).await?;
    uow.problems().upsert_problems(problems).await?;
    uow.commit().await?;
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn upsert_problem_data_creates_contest_and_problems(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;

    let contests = vec![("abc300".to_string(), "ABC".to_string())];
    let problems = vec![
        Problem {
            id: problem_id("abc300_a"),
            contest_code: contest_code("abc300"),
            problem_index: problem_index("a"),
            title: problem_title("A - Example"),
            difficulty: None,
        },
        Problem {
            id: problem_id("abc300_b"),
            contest_code: contest_code("abc300"),
            problem_index: problem_index("b"),
            title: problem_title("B - Example"),
            difficulty: None,
        },
    ];

    upsert_problem_data(&pool, &contests, &problems).await?;

    let contest_count = sqlx::query_scalar!("SELECT COUNT(*) FROM contests")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    let problem_count = sqlx::query_scalar!("SELECT COUNT(*) FROM problems")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);

    assert_eq!(contest_count, 1);
    assert_eq!(problem_count, 2);
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn upsert_problems_updates_existing_problem(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;

    let contests = vec![("abc300".to_string(), "ABC".to_string())];
    let problem = Problem {
        id: problem_id("abc300_a"),
        contest_code: contest_code("abc300"),
        problem_index: problem_index("a"),
        title: problem_title("A - Example"),
        difficulty: None,
    };
    upsert_problem_data(&pool, &contests, std::slice::from_ref(&problem)).await?;

    let updated = Problem {
        title: problem_title("A - Example modified"),
        ..problem
    };
    upsert_problem_data(&pool, &contests, &[updated]).await?;

    let title = sqlx::query_scalar!("SELECT title FROM problems WHERE id = 'abc300_a'")
        .fetch_one(&pool)
        .await?;
    assert_eq!(title, "A - Example modified");
    Ok(())
}
