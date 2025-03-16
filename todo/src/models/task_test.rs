#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = NewTasks {
            title: "Test Task".to_string(),
            content: "Test Content".to_string(),
            is_done: false,
        };

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.content, "Test Content");
        assert_eq!(task.is_done, false);
    }

    #[test]
    fn test_update_task() {
        let task = UpdateTasks {
            title: "Updated Task".to_string(),
            content: "Updated Content".to_string(),
            is_done: true,
        };

        assert_eq!(task.title, "Updated Task");
        assert_eq!(task.content, "Updated Content");
        assert_eq!(task.is_done, true);
    }
}