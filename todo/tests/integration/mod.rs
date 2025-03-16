use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use todo::models::{NewTasks, Task};
use todo::schema::tasks;

fn setup_test_db() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool")
}

#[test]
fn test_create_and_fetch_task() {
    let pool = setup_test_db();
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
    assert_eq!(inserted_task.content, "Test Content");
    assert_eq!(inserted_task.is_done, false);

    let fetched_task = tasks::table
        .find(inserted_task.id)
        .first::<Task>(&mut conn)
        .unwrap();

    assert_eq!(fetched_task.id, inserted_task.id);
    assert_eq!(fetched_task.title, inserted_task.title);
}