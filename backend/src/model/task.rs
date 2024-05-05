use super::db::Db;
use crate::model;
use crate::security::UserCtx;
use serde::{Deserialize, Serialize};
use sqlb::{HasFields, Raw};

// region:    Task Types
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Task {
	pub id: i64,
	pub cid: i64, // creator id
	pub title: String,
	pub description: String,
	pub due_date: String,
	pub typ: TaskType,
	pub status: TaskStatus,
	pub priority: TaskPriority,
}

#[derive(sqlb::Fields, Default, Debug, Clone, Deserialize)]
pub struct TaskPatch {
	pub title: Option<String>,
	pub description: Option<String>,
	pub due_date: Option<String>,
	pub typ: Option<TaskType>,
	pub status: Option<TaskStatus>,
	pub priority: Option<TaskStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "task_type_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TaskType {
	Work,
	Personal,
}
// endregion: Task Types

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "task_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TaskStatus {
	Todo,
	Bug,
	Doing,
	Testing,
	Done,
}
// endregion: Task Statuses

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "task_priority_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TaskPriority {
	Low,
	Medium,
	High,
}
// endregion: Task Priorities

sqlb::bindable!(TaskType);
sqlb::bindable!(TaskStatus);
sqlb::bindable!(TaskPriority);

// region:    TaskMac
pub struct TaskMac;

impl TaskMac {
	const TABLE: &'static str = "task";
	const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "description", "due_date", "typ", "status", "priority"];
}

// Task Model Access Controller
impl TaskMac {
	pub async fn create(db: &Db, utx: &UserCtx, data: TaskPatch) -> Result<Task, model::Error> {
		let mut fields = data.fields();
		fields.push(("cid", utx.user_id).into());
		let sb = sqlb::insert().table(Self::TABLE).data(fields).returning(Self::COLUMNS);

		let task = sb.fetch_one(db).await?;

		Ok(task)
	}

	pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Task, model::Error> {
		let sb = sqlb::select()
			.table(Self::TABLE)
			.columns(Self::COLUMNS)
			.and_where_eq("id", id);

		let result = sb.fetch_one(db).await;

		handle_fetch_one_result(result, Self::TABLE, id)
	}

	pub async fn update(db: &Db, utx: &UserCtx, id: i64, data: TaskPatch) -> Result<Task, model::Error> {
		let mut fields = data.fields();
		// augment the fields with the cid/ctime
		fields.push(("mid", utx.user_id).into());
		fields.push(("ctime", Raw("now()")).into());

		let sb = sqlb::update()
			.table(Self::TABLE)
			.data(fields)
			.and_where_eq("id", id)
			.returning(Self::COLUMNS);

		let result = sb.fetch_one(db).await;

		handle_fetch_one_result(result, Self::TABLE, id)
	}

	pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Task>, model::Error> {
		let sb = sqlb::select().table(Self::TABLE).columns(Self::COLUMNS).order_by("!id");

		// execute the query
		let tasks = sb.fetch_all(db).await?;

		Ok(tasks)
	}

	pub async fn delete(db: &Db, _utx: &UserCtx, id: i64) -> Result<Task, model::Error> {
		let sb = sqlb::delete()
			.table(Self::TABLE)
			.returning(Self::COLUMNS)
			.and_where_eq("id", id);

		let result = sb.fetch_one(db).await;

		handle_fetch_one_result(result, Self::TABLE, id)
	}
}
// endregion: TaskMac

// region:    Utils
fn handle_fetch_one_result(
	result: Result<Task, sqlx::Error>,
	typ: &'static str,
	id: i64,
) -> Result<Task, model::Error> {
	result.map_err(|sqlx_error| match sqlx_error {
		sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, id.to_string()),
		other => model::Error::SqlxError(other),
	})
}
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/model_task.rs"]
mod tests;
// endregion: Test

