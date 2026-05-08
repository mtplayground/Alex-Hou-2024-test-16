#[cfg(feature = "ssr")]
use crate::domain::todo::Todo;

#[cfg(feature = "ssr")]
use sqlx::{Executor, FromRow, Sqlite, SqlitePool};

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, Eq, FromRow, PartialEq)]
pub(crate) struct TodoRow {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) completed: bool,
    pub(crate) position: i64,
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub enum TodoStoreError {
    Database(sqlx::Error),
    InvalidTitle,
    InvalidUpdate,
    NotFound(i64),
}

#[cfg(feature = "ssr")]
impl std::fmt::Display for TodoStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(source) => write!(f, "{source}"),
            Self::InvalidTitle => write!(f, "todo title must not be empty"),
            Self::InvalidUpdate => write!(f, "update requires a title or completed value"),
            Self::NotFound(id) => write!(f, "todo with id {id} was not found"),
        }
    }
}

#[cfg(feature = "ssr")]
impl std::error::Error for TodoStoreError {}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for TodoStoreError {
    fn from(source: sqlx::Error) -> Self {
        Self::Database(source)
    }
}

#[cfg(feature = "ssr")]
pub async fn list_todos(pool: &SqlitePool) -> Result<Vec<Todo>, TodoStoreError> {
    sqlx::query_as::<_, TodoRow>(
        r#"
        SELECT id, title, completed, position
        FROM todos
        ORDER BY position ASC, id ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map(|rows| rows.into_iter().map(Todo::from).collect())
    .map_err(TodoStoreError::from)
}

#[cfg(feature = "ssr")]
pub async fn create_todo(pool: &SqlitePool, title: &str) -> Result<Todo, TodoStoreError> {
    let title = normalize_title(title)?;
    let mut tx = pool.begin().await?;

    let position = sqlx::query_scalar::<_, i64>("SELECT COALESCE(MAX(position), -1) + 1 FROM todos")
        .fetch_one(&mut *tx)
        .await?;

    let todo_id = sqlx::query("INSERT INTO todos (title, completed, position) VALUES (?1, 0, ?2)")
        .bind(&title)
        .bind(position)
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

    let todo = fetch_todo_by_id(&mut *tx, todo_id).await?;
    tx.commit().await?;

    Ok(todo)
}

#[cfg(feature = "ssr")]
pub async fn update_todo(
    pool: &SqlitePool,
    id: i64,
    title: Option<&str>,
    completed: Option<bool>,
) -> Result<Todo, TodoStoreError> {
    if title.is_none() && completed.is_none() {
        return Err(TodoStoreError::InvalidUpdate);
    }

    let normalized_title = match title {
        Some(value) => Some(normalize_title(value)?),
        None => None,
    };

    let result = sqlx::query(
        r#"
        UPDATE todos
        SET
            title = COALESCE(?2, title),
            completed = COALESCE(?3, completed),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .bind(normalized_title)
    .bind(completed)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(TodoStoreError::NotFound(id));
    }

    fetch_todo_by_id(pool, id).await
}

#[cfg(feature = "ssr")]
pub async fn delete_todo(pool: &SqlitePool, id: i64) -> Result<(), TodoStoreError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(TodoStoreError::NotFound(id));
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn toggle_all(pool: &SqlitePool, completed: bool) -> Result<u64, TodoStoreError> {
    sqlx::query(
        r#"
        UPDATE todos
        SET completed = ?1, updated_at = CURRENT_TIMESTAMP
        WHERE completed <> ?1
        "#,
    )
    .bind(completed)
    .execute(pool)
    .await
    .map(|result| result.rows_affected())
    .map_err(TodoStoreError::from)
}

#[cfg(feature = "ssr")]
pub async fn clear_completed(pool: &SqlitePool) -> Result<u64, TodoStoreError> {
    sqlx::query("DELETE FROM todos WHERE completed = 1")
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
        .map_err(TodoStoreError::from)
}

#[cfg(feature = "ssr")]
fn normalize_title(title: &str) -> Result<String, TodoStoreError> {
    let trimmed = title.trim();

    if trimmed.is_empty() {
        return Err(TodoStoreError::InvalidTitle);
    }

    Ok(trimmed.to_owned())
}

#[cfg(feature = "ssr")]
async fn fetch_todo_by_id<'e, E>(executor: E, id: i64) -> Result<Todo, TodoStoreError>
where
    E: Executor<'e, Database = Sqlite>,
{
    sqlx::query_as::<_, TodoRow>(
        r#"
        SELECT id, title, completed, position
        FROM todos
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(executor)
    .await?
    .map(Todo::from)
    .ok_or(TodoStoreError::NotFound(id))
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::db::{init_sqlite_pool, run_migrations};
    use std::{
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[tokio::test]
    async fn todo_crud_flow_works() {
        let path = temp_db_path("crud");
        let database_url = format!("sqlite://{}", path.display());
        let pool = init_sqlite_pool(&database_url).await.expect("init pool");
        run_migrations(&pool).await.expect("run migrations");

        let first = create_todo(&pool, "  first task  ").await.expect("create first");
        let second = create_todo(&pool, "second task").await.expect("create second");

        assert_eq!(first.title, "first task");
        assert_eq!(first.position, 0);
        assert_eq!(second.position, 1);

        let todos = list_todos(&pool).await.expect("list todos");
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].id, first.id);
        assert_eq!(todos[1].id, second.id);

        let updated = update_todo(&pool, first.id, Some("renamed"), Some(true))
            .await
            .expect("update todo");
        assert_eq!(updated.title, "renamed");
        assert!(updated.completed);

        delete_todo(&pool, second.id).await.expect("delete todo");

        let todos = list_todos(&pool).await.expect("list todos after delete");
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].id, first.id);

        cleanup_db(path);
    }

    #[tokio::test]
    async fn bulk_updates_report_affected_rows() {
        let path = temp_db_path("bulk");
        let database_url = format!("sqlite://{}", path.display());
        let pool = init_sqlite_pool(&database_url).await.expect("init pool");
        run_migrations(&pool).await.expect("run migrations");

        create_todo(&pool, "first").await.expect("create first");
        create_todo(&pool, "second").await.expect("create second");

        let toggled = toggle_all(&pool, true).await.expect("toggle all true");
        assert_eq!(toggled, 2);

        let cleared = clear_completed(&pool).await.expect("clear completed");
        assert_eq!(cleared, 2);
        assert!(list_todos(&pool).await.expect("list after clear").is_empty());

        cleanup_db(path);
    }

    fn temp_db_path(prefix: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "alex-hou-2024-test-16-{prefix}-{}-{timestamp}.db",
            std::process::id()
        ))
    }

    fn cleanup_db(path: PathBuf) {
        if path.exists() {
            std::fs::remove_file(path).expect("remove temp db");
        }
    }
}
