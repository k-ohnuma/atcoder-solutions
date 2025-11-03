use anyhow::Result;
use domain::model::user::{Color, Role, User};
use domain::ports::repository::problem::ProblemRepository;
use domain::ports::repository::user::UserRepository;
use domain::{
    model::{problem::Problem, solution::Solution},
    ports::repository::solution::tx::SolutionTxManager,
};
use infrastructure::ports::repository::user::UserRepositoryImpl;
use infrastructure::{
    database::ConnectionPool,
    ports::repository::{problem::ProblemRepositoryImpl, solution::tx::SolutionTransactionManager},
};
use sqlx::PgPool;
use uuid::Uuid;

#[cfg(test)]
async fn seed_contest_series(pool: &PgPool) -> Result<()> {
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

#[cfg(test)]
async fn seed_problem(problems_repo: &ProblemRepositoryImpl, problem: Problem) {
    problems_repo.create_records(vec![problem]).await.unwrap()
}

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

#[sqlx::test(migrations = "./migrations")]
async fn create_solution_records(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = ProblemRepositoryImpl::new(conn.to_owned());
    let p1 = Problem {
        id: "abc300_a".into(),
        contest_code: "abc300".into(),
        problem_index: "a".into(),
        title: "A - Example".into(),
    };
    seed_problem(&repo, p1).await;

    let repo = UserRepositoryImpl::new(conn.to_owned());
    let user = User {
        id: "id".into(),
        color: Color::default(),
        user_name: "name".into(),
        role: Role::default(),
    };
    repo.create_user(user).await?;

    let tx_mng = SolutionTransactionManager::new(conn);
    let mut uow = tx_mng.begin().await?;

    let tags = vec!["dp".to_string(), "bitset".to_string(), "dp".to_string()];

    let tag_ids = uow.tags().upsert(&tags).await?;
    assert_eq!(tag_ids.len(), 2);

    let sol_id = Uuid::now_v7();

    let solution = Solution {
        id: sol_id,
        title: "title".into(),
        problem_id: "abc300_a".into(),
        user_id: "id".into(),
        body_md: "# midashi ## what is dp?".into(),
        submit_url: "https://localhost:3000/solution".into(),
    };

    uow.solutions().create(&solution).await?;

    uow.solutions().replace_tags(sol_id, &tag_ids).await?;
    uow.solutions().replace_tags(sol_id, &tag_ids).await?;

    uow.commit().await?;

    let cnt: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solution_tags WHERE solution_id = $1")
        .bind(sol_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(cnt.0, tag_ids.len() as i64);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn create_solution_transaction_error(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let repo = ProblemRepositoryImpl::new(conn.to_owned());
    let p1 = Problem {
        id: "abc300_a".into(),
        contest_code: "abc300".into(),
        problem_index: "a".into(),
        title: "A - Example".into(),
    };
    seed_problem(&repo, p1).await;

    let repo = UserRepositoryImpl::new(conn.to_owned());
    let user = User {
        id: "id".into(),
        color: Color::default(),
        user_name: "name".into(),
        role: Role::default(),
    };
    repo.create_user(user).await?;

    let cp = ConnectionPool::new(pool.to_owned());
    let tm = SolutionTransactionManager::new(cp);
    let mut uow = tm.begin().await.unwrap();
    let tags = uow.tags().upsert(&vec!["ok".into()]).await.unwrap();
    let sol_id = Uuid::now_v7();
    let sol = Solution {
        id: sol_id,
        title: "title".into(),
        problem_id: "abc300_a".into(),
        user_id: "id".into(),
        body_md: "X".into(),
        submit_url: "https://exapmle.com".into(),
    };
    uow.solutions().create(&sol).await.unwrap();
    uow.solutions().replace_tags(sol_id, &tags).await.unwrap();

    let mut bad = tags.clone();
    // 存在していないUUIDを突っ込む
    bad.push(Uuid::now_v7());

    let _err = uow
        .solutions()
        .replace_tags(sol_id, &bad)
        .await
        .err()
        .expect("should fail");

    let cnt: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM solution_tags WHERE solution_id = $1")
        .bind(sol_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        cnt.0, 0,
        "エラーでcommitしなければ反映されない（ロールバックされる）"
    );

    Ok(())
}
