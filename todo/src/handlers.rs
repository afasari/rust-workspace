use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

use crate::models::{NewTasks, Task, UpdateTasks};
use crate::schema::tasks;

pub async fn create_task(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Json(task): Json<NewTasks>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            )
                .into_response()
        }
    };

    match diesel::insert_into(tasks::table)
        .values(&task)
        .get_result::<Task>(&mut conn)
    {
        Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create task").into_response(),
    }
}

pub async fn get_tasks(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            )
                .into_response()
        }
    };

    match tasks::table.load::<Task>(&mut conn) {
        Ok(tasks) => Json(tasks).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch tasks").into_response(),
    }
}

pub async fn get_task(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            )
                .into_response()
        }
    };

    match tasks::table.find(id).first::<Task>(&mut conn) {
        Ok(task) => Json(task).into_response(),
        Err(diesel::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch task").into_response(),
    }
}

pub async fn update_task(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Path(id): Path<i32>,
    Json(task): Json<UpdateTasks>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            )
                .into_response()
        }
    };

    match diesel::update(tasks::table.find(id))
        .set(&task)
        .execute(&mut conn)
    {
        Ok(0) => StatusCode::NOT_FOUND.into_response(),
        Ok(_) => match tasks::table.find(id).first::<Task>(&mut conn) {
            Ok(updated_task) => Json(updated_task).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch updated task",
            )
                .into_response(),
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update task").into_response(),
    }
}

pub async fn delete_task(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            )
                .into_response()
        }
    };

    match diesel::delete(tasks::table.find(id)).execute(&mut conn) {
        Ok(0) => StatusCode::NOT_FOUND.into_response(),
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete task").into_response(),
    }
}
