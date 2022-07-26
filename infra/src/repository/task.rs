use async_trait::async_trait;
use domain::{
    model::task::{Task, TaskId, TaskName},
    repository,
};
use sqlx::Row;

pub struct TaskRdbRepository {
    db_con: sqlx::SqlitePool,
}

impl TaskRdbRepository {
    pub fn new(db_con: sqlx::SqlitePool) -> Self {
        Self { db_con }
    }
}

#[async_trait]
impl repository::TaskRepository for TaskRdbRepository {
    async fn find_by_id(
        &self,
        id: &TaskId,
    ) -> Result<Option<domain::model::task::Task>, Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        let row = sqlx::query(
            "
        SELECT
            id
            ,name
            ,due_date
            ,postpone_count
        FROM tasks
        WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.db_con)
        .await?;

        if row.is_none() {
            return Ok(None);
        }

        let row = row.unwrap();
        let due_date_str: String = row.get("due_date");
        let due_date = time::OffsetDateTime::parse(
            &due_date_str,
            &time::format_description::well_known::Rfc3339,
        )
        .map_err(|e| format!("{}", e))?;
        let task = domain::model::task::Task::reconstruct(
            row.get("id"),
            TaskName::reconstruct(row.get("name")),
            row.get("postpone_count"),
            due_date,
        );

        Ok(Some(task))
    }

    async fn insert(
        &self,
        task: &mut Task,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let row = sqlx::query("INSERT INTO tasks(name, due_date, postpone_count) VALUES (?, ?, ?)")
            .bind(task.name.value.clone())
            .bind(
                task.due_date
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap(),
            )
            .bind(task.postpone_count)
            .execute(&self.db_con)
            .await?;

        let id = row.last_insert_rowid();
        task.id = Some(id);

        Ok(())
    }

    async fn update(
        &self,
        task: &Task,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let _row = sqlx::query(
            "
        UPDATE
            tasks
        SET
            name = ?
            ,due_date = ?
            ,postpone_count = ?
        WHERE
            id = ?",
        )
        .bind(task.name.value.clone())
        .bind(
            task.due_date
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap(),
        )
        .bind(task.postpone_count)
        .bind(task.id.unwrap())
        .execute(&self.db_con)
        .await?;

        Ok(())
    }
}
