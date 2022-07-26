use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub struct Provider {
    pool: SqlitePool,
}

impl Provider {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect("sqlite::memory:")
            .await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn provide_task_repository(&self) -> impl domain::repository::TaskRepository + Send + Sync {
        super::repository::TaskRdbRepository::new(self.pool.clone())
    }
}
