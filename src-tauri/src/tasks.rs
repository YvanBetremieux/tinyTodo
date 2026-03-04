use serde::{Deserialize, Serialize};

/// A single task in the todo list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub done: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub sort_order: i64,
}

/// Container for tasks.json file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksFile {
    pub tasks: Vec<Task>,
}

impl Default for TasksFile {
    fn default() -> Self {
        Self { tasks: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_serialization() {
        let task = Task {
            id: "test-uuid".to_string(),
            text: "Test task".to_string(),
            done: false,
            created_at: "2026-03-04T14:30:00Z".to_string(),
            completed_at: None,
            sort_order: 0,
        };

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "test-uuid");
        assert_eq!(deserialized.text, "Test task");
        assert!(!deserialized.done);
        assert!(deserialized.completed_at.is_none());
    }

    #[test]
    fn test_tasks_file_default_empty() {
        let file = TasksFile::default();
        assert!(file.tasks.is_empty());
    }

    #[test]
    fn test_tasks_file_serialization_roundtrip() {
        let file = TasksFile {
            tasks: vec![Task {
                id: "uuid-1".to_string(),
                text: "First task".to_string(),
                done: false,
                created_at: "2026-03-04T14:30:00Z".to_string(),
                completed_at: None,
                sort_order: 0,
            }],
        };

        let json = serde_json::to_string_pretty(&file).unwrap();
        let deserialized: TasksFile = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tasks.len(), 1);
        assert_eq!(deserialized.tasks[0].text, "First task");
    }
}
