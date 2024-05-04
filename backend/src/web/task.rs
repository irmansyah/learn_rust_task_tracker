use super::filter_auth::do_auth;
use crate::model::{Db, TaskMac, TaskPatch};
use crate::security::UserCtx;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn task_rest_filters(
	base_path: &'static str,
	db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let tasks_path = warp::path(base_path).and(warp::path("tasks"));
	let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));

	// LIST tasks `GET tasks/`
	let list = tasks_path
		.and(warp::get())
		.and(warp::path::end())
		.and(common.clone())
		.and_then(task_list);

	// GET task `GET /tasks/100`
	let get = tasks_path
		.and(warp::get())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(task_get);

	// CREATE task `POST /tasks with body TaskPatch`
	let create = tasks_path
		.and(warp::post())
		.and(common.clone())
		.and(warp::body::json())
		.and_then(task_create);

	// UPDATE task `PATCH /tasks/100 with body TaskPatch`
	let update = tasks_path
		.and(warp::patch())
		.and(common.clone())
		.and(warp::path::param())
		.and(warp::body::json())
		.and_then(task_update);

	// DELETE task `DELETE /tasks/100`
	let delete = tasks_path
		.and(warp::delete())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(task_delete);

	list.or(get).or(create).or(update).or(delete)
}

async fn task_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
	let tasks = TaskMac::list(&db, &utx).await?;
	json_response(tasks)
}

async fn task_get(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
	let task = TaskMac::get(&db, &utx, id).await?;
	json_response(task)
}

async fn task_create(db: Arc<Db>, utx: UserCtx, patch: TaskPatch) -> Result<Json, warp::Rejection> {
	let task = TaskMac::create(&db, &utx, patch).await?;
	json_response(task)
}

async fn task_update(db: Arc<Db>, utx: UserCtx, id: i64, patch: TaskPatch) -> Result<Json, warp::Rejection> {
	let task = TaskMac::update(&db, &utx, id, patch).await?;
	json_response(task)
}

async fn task_delete(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
	let task = TaskMac::delete(&db, &utx, id).await?;
	json_response(task)
}

// region:    Utils
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
	let response = json!({ "data": data });
	Ok(warp::reply::json(&response))
}
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/web_task.rs"]
mod tests;
// endregion: Test
