use anyhow::Result;
use domain::model::user::{Role, User};
use domain::ports::repository::problem::ProblemRepository;
use domain::ports::repository::user::UserRepository;
use domain::{
    error::repository::RepositoryError,
    model::{problem::Problem, solution::Solution},
    ports::repository::solution::tx::SolutionTxManager,
};
use infrastructure::ports::repository::user::UserRepositoryImpl;
use infrastructure::{
    database::ConnectionPool,
    ports::{
        repository::{problem::ProblemRepositoryImpl, solution::tx::SolutionTransactionManager},
        service::solution::SolutionServiceImpl,
    },
};
use sqlx::PgPool;
use tokio::time::{Duration, sleep};
use usecase::{model::solution::SolutionListSort, service::solution::SolutionService};
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
        user_name: "name".into(),
        role: Role::default(),
    };
    repo.create_user(user).await?;

    let cp = ConnectionPool::new(pool.to_owned());
    let tm = SolutionTransactionManager::new(cp);
    let mut uow = tm.begin().await.unwrap();
    let tags = uow.tags().upsert(&["ok".into()]).await.unwrap();
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

    uow.solutions()
        .replace_tags(sol_id, &bad)
        .await
        .expect_err("should fail");

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

#[cfg(test)]
async fn create_solution_only(
    conn: ConnectionPool,
    problem_id: &str,
    user_id: &str,
    title: &str,
) -> Result<Uuid> {
    let tx_mng = SolutionTransactionManager::new(conn);
    let mut uow = tx_mng.begin().await?;
    let solution_id = Uuid::now_v7();
    let solution = Solution {
        id: solution_id,
        title: title.to_string(),
        problem_id: problem_id.to_string(),
        user_id: user_id.to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
    };
    uow.solutions().create(&solution).await?;
    uow.commit().await?;
    Ok(solution_id)
}

#[cfg(test)]
async fn insert_vote(pool: &PgPool, user_id: &str, solution_id: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO solution_votes (user_id, solution_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, solution_id) DO NOTHING
        "#,
        user_id,
        solution_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn get_solutions_by_problem_id_sorts_latest_and_votes(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let problems_repo = ProblemRepositoryImpl::new(conn.to_owned());
    seed_problem(
        &problems_repo,
        Problem {
            id: "abc300_a".into(),
            contest_code: "abc300".into(),
            problem_index: "a".into(),
            title: "A - Example".into(),
        },
    )
    .await;

    let user_repo = UserRepositoryImpl::new(conn.to_owned());
    for (id, name) in [
        ("author1", "author1"),
        ("author2", "author2"),
        ("voter1", "voter1"),
        ("voter2", "voter2"),
        ("voter3", "voter3"),
    ] {
        user_repo
            .create_user(User {
                id: id.to_string(),
                user_name: name.to_string(),
                role: Role::default(),
            })
            .await?;
    }

    let old_solution_id =
        create_solution_only(conn.to_owned(), "abc300_a", "author1", "old").await?;
    sleep(Duration::from_millis(20)).await;
    let new_solution_id =
        create_solution_only(conn.to_owned(), "abc300_a", "author2", "new").await?;

    insert_vote(&pool, "voter1", old_solution_id).await?;
    insert_vote(&pool, "voter2", old_solution_id).await?;
    insert_vote(&pool, "voter3", new_solution_id).await?;

    let service = SolutionServiceImpl::new(conn);

    let latest = service
        .get_solutions_by_problem_id("abc300_a".to_string(), SolutionListSort::Latest)
        .await?;
    assert_eq!(latest.len(), 2);
    assert_eq!(latest[0].id, new_solution_id);
    assert_eq!(latest[1].id, old_solution_id);

    let votes = service
        .get_solutions_by_problem_id("abc300_a".to_string(), SolutionListSort::Votes)
        .await?;
    assert_eq!(votes.len(), 2);
    assert_eq!(votes[0].id, old_solution_id);
    assert_eq!(votes[0].votes_count, 2);
    assert_eq!(votes[1].id, new_solution_id);
    assert_eq!(votes[1].votes_count, 1);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn get_solutions_by_user_name_votes_returns_problem_title_and_vote_count(
    pool: PgPool,
) -> Result<()> {
    seed_contest_series(&pool).await?;
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let problems_repo = ProblemRepositoryImpl::new(conn.to_owned());
    seed_problem(
        &problems_repo,
        Problem {
            id: "abc300_a".into(),
            contest_code: "abc300".into(),
            problem_index: "a".into(),
            title: "A - Example".into(),
        },
    )
    .await;
    seed_problem(
        &problems_repo,
        Problem {
            id: "abc300_b".into(),
            contest_code: "abc300".into(),
            problem_index: "b".into(),
            title: "B - Example".into(),
        },
    )
    .await;

    let user_repo = UserRepositoryImpl::new(conn.to_owned());
    for (id, name) in [
        ("author", "alice"),
        ("voter1", "voter1"),
        ("voter2", "voter2"),
        ("voter3", "voter3"),
    ] {
        user_repo
            .create_user(User {
                id: id.to_string(),
                user_name: name.to_string(),
                role: Role::default(),
            })
            .await?;
    }

    let a_id = create_solution_only(conn.to_owned(), "abc300_a", "author", "solution-a").await?;
    let b_id = create_solution_only(conn.to_owned(), "abc300_b", "author", "solution-b").await?;

    insert_vote(&pool, "voter1", b_id).await?;
    insert_vote(&pool, "voter2", b_id).await?;
    insert_vote(&pool, "voter3", a_id).await?;

    let service = SolutionServiceImpl::new(conn);
    let rows = service
        .get_solutions_by_user_name("alice".to_string(), SolutionListSort::Votes)
        .await?;

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].problem_id, "abc300_b");
    assert_eq!(rows[0].problem_title, "B - Example");
    assert_eq!(rows[0].votes_count, 2);
    assert_eq!(rows[1].problem_id, "abc300_a");
    assert_eq!(rows[1].problem_title, "A - Example");
    assert_eq!(rows[1].votes_count, 1);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn get_solutions_by_problem_id_votes_tie_breaks_by_created_at_desc(
    pool: PgPool,
) -> Result<()> {
    seed_contest_series(&pool).await?;
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let problems_repo = ProblemRepositoryImpl::new(conn.to_owned());
    seed_problem(
        &problems_repo,
        Problem {
            id: "abc301_a".into(),
            contest_code: "abc301".into(),
            problem_index: "a".into(),
            title: "A - Example".into(),
        },
    )
    .await;

    let user_repo = UserRepositoryImpl::new(conn.to_owned());
    for (id, name) in [
        ("author1", "author1"),
        ("author2", "author2"),
        ("voter1", "voter1"),
        ("voter2", "voter2"),
    ] {
        user_repo
            .create_user(User {
                id: id.to_string(),
                user_name: name.to_string(),
                role: Role::default(),
            })
            .await?;
    }

    let older = create_solution_only(conn.to_owned(), "abc301_a", "author1", "older").await?;
    sleep(Duration::from_millis(20)).await;
    let newer = create_solution_only(conn.to_owned(), "abc301_a", "author2", "newer").await?;

    insert_vote(&pool, "voter1", older).await?;
    insert_vote(&pool, "voter2", newer).await?;

    let service = SolutionServiceImpl::new(conn);
    let rows = service
        .get_solutions_by_problem_id("abc301_a".to_string(), SolutionListSort::Votes)
        .await?;

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].id, newer);
    assert_eq!(rows[1].id, older);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn create_solution_with_missing_problem_returns_fk_violation(pool: PgPool) -> Result<()> {
    seed_roles(&pool).await?;
    let conn = ConnectionPool::new(pool.clone());
    let user_repo = UserRepositoryImpl::new(conn.to_owned());
    user_repo
        .create_user(User {
            id: "author".to_string(),
            user_name: "author".to_string(),
            role: Role::default(),
        })
        .await?;

    let tx_mng = SolutionTransactionManager::new(conn);
    let mut uow = tx_mng.begin().await?;
    let solution = Solution {
        id: Uuid::now_v7(),
        title: "title".to_string(),
        problem_id: "missing_problem".to_string(),
        user_id: "author".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
    };

    let err = uow
        .solutions()
        .create(&solution)
        .await
        .expect_err("fk violation expected");
    assert!(matches!(err, RepositoryError::ForeignKeyViolation(_)));
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn deleting_user_cascades_solutions_comments_and_votes(pool: PgPool) -> Result<()> {
    seed_contest_series(&pool).await?;
    seed_roles(&pool).await?;

    let conn = ConnectionPool::new(pool.clone());
    let problems_repo = ProblemRepositoryImpl::new(conn.to_owned());
    seed_problem(
        &problems_repo,
        Problem {
            id: "abc302_a".into(),
            contest_code: "abc302".into(),
            problem_index: "a".into(),
            title: "A - Example".into(),
        },
    )
    .await;

    let user_repo = UserRepositoryImpl::new(conn.to_owned());
    user_repo
        .create_user(User {
            id: "author".to_string(),
            user_name: "author".to_string(),
            role: Role::default(),
        })
        .await?;
    user_repo
        .create_user(User {
            id: "viewer".to_string(),
            user_name: "viewer".to_string(),
            role: Role::default(),
        })
        .await?;

    let solution_id = create_solution_only(conn.to_owned(), "abc302_a", "author", "title").await?;

    sqlx::query!(
        r#"
        INSERT INTO comments (user_id, solution_id, body_md)
        VALUES ($1, $2, $3)
        "#,
        "author",
        solution_id,
        "comment body"
    )
    .execute(&pool)
    .await?;

    insert_vote(&pool, "viewer", solution_id).await?;

    let solution_count_before = sqlx::query_scalar!("SELECT COUNT(*) FROM solutions")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    let comment_count_before = sqlx::query_scalar!("SELECT COUNT(*) FROM comments")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    let vote_count_before = sqlx::query_scalar!("SELECT COUNT(*) FROM solution_votes")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    assert_eq!(solution_count_before, 1);
    assert_eq!(comment_count_before, 1);
    assert_eq!(vote_count_before, 1);

    user_repo.delete_by_uid("author").await?;

    let solution_count_after = sqlx::query_scalar!("SELECT COUNT(*) FROM solutions")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    let comment_count_after = sqlx::query_scalar!("SELECT COUNT(*) FROM comments")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    let vote_count_after = sqlx::query_scalar!("SELECT COUNT(*) FROM solution_votes")
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);
    assert_eq!(solution_count_after, 0);
    assert_eq!(comment_count_after, 0);
    assert_eq!(vote_count_after, 0);

    Ok(())
}
