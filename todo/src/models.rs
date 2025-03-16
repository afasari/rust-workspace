use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_done: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTasks {
    pub title: String,
    pub content: String,
    pub is_done: bool,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdateTasks {
    pub title: String,
    pub content: String,
    pub is_done: bool,
}
