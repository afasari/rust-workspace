use reqwest;
use serde_json::json;
use todo::models::Task;

#[tokio::test]
async fn test_task_crud() {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:8081";

    // Create a new task
    let response = client
        .post(&format!("{}/tasks", base_url))
        .json(&json!({
            "title": "E2E Test Task",
            "content": "E2E Test Content",
            "is_done": false
        }))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    let created_task: Task = response.json().await.unwrap();

    // Verify task was created
    let response = client
        .get(&format!("{}/tasks/{}", base_url, created_task.id))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    let task: Task = response.json().await.unwrap();
    assert_eq!(task.title, "E2E Test Task");

    // Clean up
    let response = client
        .delete(&format!("{}/tasks/{}", base_url, created_task.id))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}