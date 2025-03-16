mod helpers;

use diesel::prelude::*;
use todo::models::{NewTasks, Task};
use todo::schema::tasks;

#[test]
fn test_create_and_fetch_task() {
    let pool = helpers::setup_test_db();
    let mut conn = pool.get().unwrap();

    // Clean up before test
    diesel::delete(tasks::table).execute(&mut conn).unwrap();

    let new_task = NewTasks {
        title: "Test Task".to_string(),
        content: "Test Content".to_string(),
        is_done: false,
    };

    let inserted_task = diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result::<Task>(&mut conn)
        .unwrap();

    assert_eq!(inserted_task.title, "Test Task");
}
