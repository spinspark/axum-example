use crate::sqlite::establish_pool;
use rand::RngExt;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct TestContext {
    database_path: String,
    pool: SqlitePool,
}

impl TestContext {
    pub async fn new() -> Self {
        let number: u32 = {
            let mut rng = rand::rng();
            rng.random()
        };

        let database_path = format!(".__diesel_test_{number}.db");
        let pool = establish_pool(&database_path)
            .await
            .expect("Failed to create SQLite database");

        let fixtures = include_str!("../../migrations/test/fixtures.sql");
        sqlx::query(fixtures)
            .execute(&pool)
            .await
            .expect("Failed to insert test data");

        Self {
            database_path,
            pool,
        }
    }

    pub const fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        std::fs::remove_file(&self.database_path).expect("Failed to delete test database");
        std::fs::remove_file(format!("{}-shm", self.database_path))
            .expect("Failed to delete test database");
        std::fs::remove_file(format!("{}-wal", self.database_path))
            .expect("Failed to delete test database");
    }
}
