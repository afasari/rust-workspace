use reqwest;
use serde_json::json;
use tokio;

#[tokio::test]
async fn test_crud_operations() {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:8081";

    // Create task
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
    let task = response.json::<serde_json::Value>().await.unwrap();
    let task_id = task["id"].as_i64().unwrap();

    // Get task
    let response = client
        .get(&format!("{}/tasks/{}", base_url, task_id))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    let task = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(task["title"], "E2E Test Task");

    // Update task
    let response = client
        .post(&format!("{}/tasks/{}", base_url, task_id))
        .json(&json!({
            "title": "Updated E2E Task",
            "content": "Updated E2E Content",
            "is_done": true
        }))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    // Delete task
    let response = client
        .delete(&format!("{}/tasks/{}", base_url, task_id))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}