use super::{Task, TaskMac};
use crate::model;
use crate::model::db::init_db;
use crate::model::task::{TaskPatch, TaskStatus};
use crate::security::utx_from_token;

#[tokio::test]
async fn model_task_create() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;
	let data_fx = TaskPatch {
		due_date: Some("test - model_task_create 1".to_string()),
		..Default::default()
	};

	// -- ACTION
	let task_created = TaskMac::create(&db, &utx, data_fx.clone()).await?;

	// -- CHECK
	assert!(task_created.id >= 1000, "Id should be >= 1000");
	assert_eq!(data_fx.description.unwrap(), task_created.description);
	assert_eq!(TaskStatus::Todo, task_created.status);

	Ok(())
}

#[tokio::test]
async fn model_task_get_ok() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let task = TaskMac::get(&db, &utx, 100).await?;

	// -- CHECK
	assert_eq!(100, task.id);
	assert_eq!("task 100", task.title);
	assert_eq!(TaskStatus::Todo, task.status);

	Ok(())
}

#[tokio::test]
async fn model_task_get_wong_id() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let result = TaskMac::get(&db, &utx, 999).await;

	// -- CHECK
	match result {
		Ok(_) => assert!(false, "Should not succeed"),
		Err(model::Error::EntityNotFound(typ, id)) => {
			assert_eq!("task", typ);
			assert_eq!(999.to_string(), id);
		}
		other_error => assert!(false, "Wrong Error {:?} ", other_error),
	}

	Ok(())
}

#[tokio::test]
async fn model_task_update_ok() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;
	let data_fx = TaskPatch {
		due_date: Some("test - model_task_update_ok 1".to_string()),
		..Default::default()
	};
	let task_fx = TaskMac::create(&db, &utx, data_fx.clone()).await?;
	let update_data_fx = TaskPatch {
		due_date: Some("test - model_task_update_ok 2".to_string()),
		..Default::default()
	};

	// -- ACTION
	let task_updated = TaskMac::update(&db, &utx, task_fx.id, update_data_fx.clone()).await?;

	// -- CHECK
	let tasks = TaskMac::list(&db, &utx).await?;
	assert_eq!(3, tasks.len());
	assert_eq!(task_fx.id, task_updated.id);
	assert_eq!(update_data_fx.title.unwrap(), task_updated.title);

	Ok(())
}

#[tokio::test]
async fn model_task_list() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let tasks = TaskMac::list(&db, &utx).await?;

	// -- CHECK
	assert_eq!(2, tasks.len());
	// task 101
	assert_eq!(101, tasks[0].id);
	assert_eq!(123, tasks[0].cid);
	assert_eq!("task 101", tasks[0].title);
	// task 100
	assert_eq!(100, tasks[1].id);
	assert_eq!(123, tasks[1].cid);
	assert_eq!("task 100", tasks[1].title);

	Ok(())
}

#[tokio::test]
async fn model_task_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let task = TaskMac::delete(&db, &utx, 100).await?;

	// -- CHECK - deleted item
	assert_eq!(100, task.id);
	assert_eq!("task 100", task.title);

	// -- CHECK - list
	let tasks: Vec<Task> = sqlb::select().table("task").fetch_all(&db).await?;
	assert_eq!(1, tasks.len());

	Ok(())
}
