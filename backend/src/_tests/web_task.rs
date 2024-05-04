use super::task_rest_filters;
use crate::model::{init_db, Task, TaskMac, TaskStatus};
use crate::security::utx_from_token;
use crate::web::handle_rejection;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
use std::str::from_utf8;
use std::sync::Arc;
use warp::hyper::body::Bytes;
use warp::hyper::Response;
use warp::Filter;

#[tokio::test]
async fn web_task_list() -> Result<()> {
	// -- FIXTURE
	let db = init_db().await?;
	let db = Arc::new(db);
	let task_apis = task_rest_filters("api", db.clone()).recover(handle_rejection);

	// -- ACTION
	let resp = warp::test::request()
		.method("GET")
		.header("X-Auth-Token", "123")
		.path("/api/tasks")
		.reply(&task_apis)
		.await;

	// -- CHECK
	assert_eq!(200, resp.status(), "http status");

	// extract response .data
	let tasks: Vec<Task> = extract_body_data(resp)?;

	// -- CHECK - tasks
	assert_eq!(2, tasks.len(), "number of tasks");
	assert_eq!(101, tasks[0].id);
	assert_eq!("task 101", tasks[0].title);
	assert_eq!(TaskStatus::Done, tasks[0].status);

	Ok(())
}

#[tokio::test]
async fn web_task_get_ok() -> Result<()> {
	// -- FIXTURE
	let db = init_db().await?;
	let db = Arc::new(db);
	let task_apis = task_rest_filters("api", db).recover(handle_rejection);

	// -- ACTION
	let resp = warp::test::request()
		.method("GET")
		.header("X-Auth-Token", "123")
		.path("/api/tasks/100")
		.reply(&task_apis)
		.await;

	// -- CHECK - status
	assert_eq!(200, resp.status(), "http status");

	// extract response .data
	let task: Task = extract_body_data(resp)?;

	// -- CHECK - .data (task)
	assert_eq!(100, task.id);
	assert_eq!("task 100", task.title);
	assert_eq!(TaskStatus::Todo, task.status);

	Ok(())
}

#[tokio::test]
async fn web_task_create_ok() -> Result<()> {
	// -- FIXTURE
	let db = init_db().await?;
	let db = Arc::new(db);
	let task_apis = task_rest_filters("api", db.clone()).recover(handle_rejection);
	// new task fixture
	const TITLE: &str = "test - web_task_create_ok";
	const DUE_DATE: &str = "test - web_task_create_ok";
	let body = json!({
		"title": TITLE,
		"due_date": DUE_DATE,
	});

	// -- ACTION
	let resp = warp::test::request()
		.method("POST")
		.header("X-Auth-Token", "123")
		.path("/api/tasks")
		.json(&body)
		.reply(&task_apis)
		.await;

	// -- CHECK - status
	assert_eq!(200, resp.status(), "http status");

	// extract response .data
	let task: Task = extract_body_data(resp)?;

	// -- CHECK - .data (task)
	assert!(task.id >= 1000, "task.id should be >= to 1000");
	assert_eq!(TITLE, task.title);
	assert_eq!(TaskStatus::Done, task.status);

	Ok(())
}

#[tokio::test]
async fn web_task_update_ok() -> Result<()> {
	// -- FIXTURE
	let db = init_db().await?;
	let db = Arc::new(db);
	let task_apis = task_rest_filters("api", db.clone()).recover(handle_rejection);
	// udpated task
	const TITLE: &str = "test - task 100 updated";
	let body = json!({
		"title": TITLE,
		"status": "Done"
	});

	// -- ACTION
	let resp = warp::test::request()
		.method("PATCH")
		.header("X-Auth-Token", "123")
		.path("/api/tasks/100")
		.json(&body)
		.reply(&task_apis)
		.await;

	// -- CHECK - status
	assert_eq!(200, resp.status(), "http status");

	// extract response .data
	let task: Task = extract_body_data(resp)?;

	// -- CHECK - .data (task)
	assert_eq!(100, task.id, "task.id");
	assert_eq!(TITLE, task.title);
	assert_eq!(TaskStatus::Done, task.status);

	Ok(())
}

#[tokio::test]
async fn web_task_delete_ok() -> Result<()> {
	// -- FIXTURE
	let db = init_db().await?;
	let db = Arc::new(db);
	let task_apis = task_rest_filters("api", db.clone()).recover(handle_rejection);

	// -- ACTION
	let resp = warp::test::request()
		.method("DELETE")
		.header("X-Auth-Token", "123")
		.path("/api/tasks/100")
		.reply(&task_apis)
		.await;

	// -- CHECK - status
	assert_eq!(200, resp.status(), "http status");

	// extract response .data
	let task: Task = extract_body_data(resp)?;

	// -- CHECK - .data (tasks)
	assert_eq!(100, task.id);
	assert_eq!("task 100", task.title);
	assert_eq!(TaskStatus::Todo, task.status);

	// -- CHECK - list .len() should be 1
	let utx = utx_from_token(&db, "123").await?;
	let tasks = TaskMac::list(&db, &utx).await?;
	assert_eq!(1, tasks.len(), "tasks length");
	assert_eq!(101, tasks[0].id, "Task remaining should be 101");

	Ok(())
}

// region:    Web Test Utils
fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
	for<'de> D: Deserialize<'de>,
{
	// parse the body as serde_json::Value
	let body = from_utf8(resp.body())?;
	let mut body: Value =
		from_str(body).with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;

	// extract the data
	let data = body["data"].take();

	// deserialize the data to D
	let data: D = from_value(data)?;

	Ok(data)
}
// endregion: Web Test Utils
