use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize, FromRow)]
pub struct Tag {
    id: i32,
    label: String,
}

impl Tag {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT id, label FROM tags")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT id, label FROM tags WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_label(
        label: &str,
        pool: &SqlitePool,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT id, label FROM tags WHERE label = ?")
            .bind(label)
            .fetch_optional(pool)
            .await
    }

    pub async fn delete_by_id(id: i32, pool: &SqlitePool) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[derive(Debug, Deserialize)]
pub struct NewTag {
    pub label: String,
}

impl NewTag {
    pub async fn insert(self, pool: &SqlitePool) -> Result<Tag, sqlx::Error> {
        sqlx::query_as("INSERT INTO tags (label) VALUES (?) RETURNING id, label")
            .bind(&self.label)
            .fetch_one(pool)
            .await
    }
}
