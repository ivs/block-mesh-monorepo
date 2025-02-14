use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::aggregate::{Aggregate, AggregateName};

#[tracing::instrument(
    name = "get_or_create_aggregate_by_user_and_name_no_transaction",
    skip(pool),
    level = "trace",
    ret,
    err
)]
pub(crate) async fn get_or_create_aggregate_by_user_and_name_no_transaction(
    pool: &PgPool,
    name: AggregateName,
    user_id: Uuid,
) -> anyhow::Result<Aggregate> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let value = serde_json::Value::Null;
    let aggregate = sqlx::query_as!(
        Aggregate,
        r#"
        WITH
            extant AS (
                SELECT id, user_id, name, value, created_at FROM aggregates WHERE (user_id, name) = ($3, $4)
            ),
            inserted AS (
                INSERT INTO aggregates (id , created_at, user_id, name, value)
                SELECT $1, $2, $3, $4,  CAST( $5 as JSONB )
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id, user_id, name, value, created_at
            )
        SELECT id, user_id, name, value, created_at FROM inserted
        UNION ALL
        SELECT id, user_id, name, value, created_at FROM extant
        "#,
        id,
        now,
        user_id,
        name.to_string(),
        value
    )
    .fetch_one(pool)
    .await?;
    Ok(aggregate)
}
