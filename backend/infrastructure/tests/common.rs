use anyhow::Result;
use sqlx::PgPool;

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
