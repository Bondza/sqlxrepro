use sqlx::PgConnection;
use sqlx::{types::chrono::NaiveDate, Connection};

struct Ab {
    id: i64,
    name: Option<String>,
    a_name: String,
    date: NaiveDate,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn =
        PgConnection::connect("postgres://postgres:postgres@localhost:5433/postgres").await?;

    let result_ok = sqlx::query_as!(
        Ab,
        "SELECT
            b.id,
            b.name,
            a.name AS a_name,
            a.date
        FROM b JOIN a ON b.a_id = a.id
        ORDER BY a.date DESC"
    )
    .fetch_one(&mut conn)
    .await?;

    let result_err = sqlx::query_as!(
        Ab,
        "SELECT
            b.id,
            b.name,
            a.name AS a_name,
            a.date
        FROM b JOIN a ON b.a_id = a.id"
    )
    .fetch_one(&mut conn)
    .await?;

    Ok(())
}
