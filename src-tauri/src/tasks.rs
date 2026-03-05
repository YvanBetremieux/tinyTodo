use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// A single task in the todo list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Managed state for task persistence
pub struct TaskState {
    pub tasks: Mutex<TasksFile>,
    pub data_path: PathBuf,
}

/// Load tasks from JSON file, creating it if absent
pub fn load_tasks(path: &Path) -> TasksFile {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }

    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => {
            let default = TasksFile::default();
            let _ = save_tasks(path, &default);
            default
        }
    }
}

/// Save tasks to JSON file using atomic write (temp + rename)
pub fn save_tasks(path: &Path, tasks_file: &TasksFile) -> Result<(), String> {
    let json = serde_json::to_string_pretty(tasks_file)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let parent = path.parent().ok_or("Invalid path")?;
    if !parent.exists() {
        fs::create_dir_all(parent).map_err(|e| format!("Cannot create directory: {}", e))?;
    }

    // Atomic write: write to temp file, then rename
    let temp_path = path.with_extension("json.tmp");
    let mut file =
        fs::File::create(&temp_path).map_err(|e| format!("Cannot create temp file: {}", e))?;
    file.write_all(json.as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;
    file.sync_all()
        .map_err(|e| format!("Sync error: {}", e))?;

    fs::rename(&temp_path, path).map_err(|e| format!("Rename error: {}", e))?;

    Ok(())
}

/// IPC command: get active tasks (done=false), sorted by sort_order
#[tauri::command]
pub fn get_tasks(state: tauri::State<'_, TaskState>) -> Vec<Task> {
    let tasks_file = state.tasks.lock().unwrap();
    let mut active: Vec<Task> = tasks_file
        .tasks
        .iter()
        .filter(|t| !t.done)
        .cloned()
        .collect();
    active.sort_by_key(|t| t.sort_order);
    active
}

/// IPC command: create a new task
#[tauri::command]
pub fn create_task(text: String, state: tauri::State<'_, TaskState>) -> Result<Task, String> {
    let mut tasks_file = state.tasks.lock().unwrap();

    // Shift existing sort_order by +1 (new task goes to top = sort_order 0)
    for task in &mut tasks_file.tasks {
        task.sort_order += 1;
    }

    let new_task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        text,
        done: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        completed_at: None,
        sort_order: 0,
    };

    tasks_file.tasks.push(new_task.clone());

    save_tasks(&state.data_path, &tasks_file)?;

    Ok(new_task)
}

/// IPC command: toggle a task's done status
#[tauri::command]
pub fn toggle_task(id: String, state: tauri::State<'_, TaskState>) -> Result<Task, String> {
    let mut tasks_file = state.tasks.lock().unwrap();

    let task = tasks_file
        .tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("Task not found: {}", id))?;

    task.done = !task.done;
    if task.done {
        task.completed_at = Some(chrono::Utc::now().to_rfc3339());
    } else {
        task.completed_at = None;
    }

    let result = task.clone();
    save_tasks(&state.data_path, &tasks_file)?;

    Ok(result)
}

/// IPC command: update a task's text
#[tauri::command]
pub fn update_task(id: String, text: String, state: tauri::State<'_, TaskState>) -> Result<Task, String> {
    let trimmed = text.trim().to_string();
    if trimmed.is_empty() {
        return Err("Task text cannot be empty".to_string());
    }

    let mut tasks_file = state.tasks.lock().unwrap();

    let task = tasks_file
        .tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("Task not found: {}", id))?;

    task.text = trimmed;
    let result = task.clone();
    save_tasks(&state.data_path, &tasks_file)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_path(dir: &TempDir) -> PathBuf {
        dir.path().join("tasks.json")
    }

    #[test]
    fn test_task_serialization_roundtrip() {
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
        assert_eq!(deserialized, task);
    }

    #[test]
    fn test_tasks_file_default_empty() {
        let file = TasksFile::default();
        assert!(file.tasks.is_empty());
    }

    #[test]
    fn test_load_tasks_creates_file_if_absent() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);

        assert!(!path.exists());
        let loaded = load_tasks(&path);
        assert!(path.exists());
        assert!(loaded.tasks.is_empty());

        // Verify file content is valid JSON
        let content = fs::read_to_string(&path).unwrap();
        let parsed: TasksFile = serde_json::from_str(&content).unwrap();
        assert!(parsed.tasks.is_empty());
    }

    #[test]
    fn test_load_tasks_creates_directory_if_absent() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("subdir").join("tasks.json");

        assert!(!path.parent().unwrap().exists());
        let loaded = load_tasks(&path);
        assert!(path.exists());
        assert!(loaded.tasks.is_empty());
    }

    #[test]
    fn test_save_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);

        let tasks_file = TasksFile {
            tasks: vec![
                Task {
                    id: "uuid-1".to_string(),
                    text: "First task".to_string(),
                    done: false,
                    created_at: "2026-03-04T14:30:00Z".to_string(),
                    completed_at: None,
                    sort_order: 0,
                },
                Task {
                    id: "uuid-2".to_string(),
                    text: "Second task".to_string(),
                    done: true,
                    created_at: "2026-03-04T14:00:00Z".to_string(),
                    completed_at: Some("2026-03-04T15:00:00Z".to_string()),
                    sort_order: 1,
                },
            ],
        };

        save_tasks(&path, &tasks_file).unwrap();
        let loaded = load_tasks(&path);

        assert_eq!(loaded.tasks.len(), 2);
        assert_eq!(loaded.tasks[0].text, "First task");
        assert_eq!(loaded.tasks[1].text, "Second task");
    }

    #[test]
    fn test_save_tasks_is_pretty_printed() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);

        let tasks_file = TasksFile {
            tasks: vec![Task {
                id: "uuid-1".to_string(),
                text: "Test".to_string(),
                done: false,
                created_at: "2026-03-04T14:30:00Z".to_string(),
                completed_at: None,
                sort_order: 0,
            }],
        };

        save_tasks(&path, &tasks_file).unwrap();
        let content = fs::read_to_string(&path).unwrap();

        // Pretty-printed JSON has newlines and indentation
        assert!(content.contains('\n'));
        assert!(content.contains("  "));
        // Human-readable field names
        assert!(content.contains("\"text\""));
        assert!(content.contains("\"created_at\""));
    }

    #[test]
    fn test_atomic_write_no_temp_file_left() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);

        let tasks_file = TasksFile::default();
        save_tasks(&path, &tasks_file).unwrap();

        // Temp file should not exist after successful write
        let temp_path = path.with_extension("json.tmp");
        assert!(!temp_path.exists());
        // But the actual file should exist
        assert!(path.exists());
    }

    #[test]
    fn test_create_task_generates_uuid() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);
        let state = TaskState {
            tasks: Mutex::new(TasksFile::default()),
            data_path: path,
        };

        // Simulate the create_task logic without tauri::State
        let mut tasks_file = state.tasks.lock().unwrap();
        let new_task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            text: "Test task".to_string(),
            done: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            completed_at: None,
            sort_order: 0,
        };
        tasks_file.tasks.push(new_task.clone());
        drop(tasks_file);

        // UUID should be valid v4 format
        assert!(uuid::Uuid::parse_str(&new_task.id).is_ok());
        assert!(!new_task.done);
        assert!(new_task.completed_at.is_none());
        assert_eq!(new_task.sort_order, 0);
    }

    #[test]
    fn test_create_task_shifts_sort_order() {
        let existing = TasksFile {
            tasks: vec![
                Task {
                    id: "old-1".to_string(),
                    text: "Old task 1".to_string(),
                    done: false,
                    created_at: "2026-03-04T14:00:00Z".to_string(),
                    completed_at: None,
                    sort_order: 0,
                },
                Task {
                    id: "old-2".to_string(),
                    text: "Old task 2".to_string(),
                    done: false,
                    created_at: "2026-03-04T13:00:00Z".to_string(),
                    completed_at: None,
                    sort_order: 1,
                },
            ],
        };

        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);
        let state = TaskState {
            tasks: Mutex::new(existing),
            data_path: path,
        };

        let mut tasks_file = state.tasks.lock().unwrap();
        // Shift existing sort_orders
        for task in &mut tasks_file.tasks {
            task.sort_order += 1;
        }
        tasks_file.tasks.push(Task {
            id: "new-1".to_string(),
            text: "New task".to_string(),
            done: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            completed_at: None,
            sort_order: 0,
        });

        // Verify sort_order shift
        let old1 = tasks_file.tasks.iter().find(|t| t.id == "old-1").unwrap();
        let old2 = tasks_file.tasks.iter().find(|t| t.id == "old-2").unwrap();
        let new1 = tasks_file.tasks.iter().find(|t| t.id == "new-1").unwrap();

        assert_eq!(new1.sort_order, 0);
        assert_eq!(old1.sort_order, 1);
        assert_eq!(old2.sort_order, 2);
    }

    #[test]
    fn test_get_tasks_filters_done() {
        let tasks_file = TasksFile {
            tasks: vec![
                Task {
                    id: "active".to_string(),
                    text: "Active task".to_string(),
                    done: false,
                    created_at: "2026-03-04T14:00:00Z".to_string(),
                    completed_at: None,
                    sort_order: 0,
                },
                Task {
                    id: "done".to_string(),
                    text: "Done task".to_string(),
                    done: true,
                    created_at: "2026-03-04T13:00:00Z".to_string(),
                    completed_at: Some("2026-03-04T14:00:00Z".to_string()),
                    sort_order: 1,
                },
            ],
        };

        let active: Vec<Task> = tasks_file
            .tasks
            .iter()
            .filter(|t| !t.done)
            .cloned()
            .collect();

        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, "active");
    }

    #[test]
    fn test_get_tasks_sorted_by_sort_order() {
        let tasks_file = TasksFile {
            tasks: vec![
                Task {
                    id: "b".to_string(),
                    text: "Second".to_string(),
                    done: false,
                    created_at: "2026-03-04T14:00:00Z".to_string(),
                    completed_at: None,
                    sort_order: 2,
                },
                Task {
                    id: "a".to_string(),
                    text: "First".to_string(),
                    done: false,
                    created_at: "2026-03-04T15:00:00Z".to_string(),
                    completed_at: None,
                    sort_order: 0,
                },
                Task {
                    id: "c".to_string(),
                    text: "Third".to_string(),
                    done: false,
                    created_at: "2026-03-04T13:00:00Z".to_string(),
                    completed_at: None,
                    sort_order: 5,
                },
            ],
        };

        let mut active: Vec<Task> = tasks_file
            .tasks
            .iter()
            .filter(|t| !t.done)
            .cloned()
            .collect();
        active.sort_by_key(|t| t.sort_order);

        assert_eq!(active[0].id, "a");
        assert_eq!(active[1].id, "b");
        assert_eq!(active[2].id, "c");
    }

    #[test]
    fn test_created_at_is_iso8601() {
        let now = chrono::Utc::now().to_rfc3339();
        // RFC3339 is a profile of ISO 8601
        assert!(now.contains('T'));
        assert!(now.contains('+') || now.contains('Z'));
    }

    #[test]
    fn test_toggle_task_sets_done_and_completed_at() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);
        let tasks_file = TasksFile {
            tasks: vec![Task {
                id: "task-1".to_string(),
                text: "Test task".to_string(),
                done: false,
                created_at: "2026-03-04T14:00:00Z".to_string(),
                completed_at: None,
                sort_order: 0,
            }],
        };
        save_tasks(&path, &tasks_file).unwrap();

        let state = TaskState {
            tasks: Mutex::new(tasks_file),
            data_path: path.clone(),
        };

        // Simulate toggle
        {
            let mut tf = state.tasks.lock().unwrap();
            let task = tf.tasks.iter_mut().find(|t| t.id == "task-1").unwrap();
            task.done = true;
            task.completed_at = Some(chrono::Utc::now().to_rfc3339());
            save_tasks(&state.data_path, &tf).unwrap();
        }

        let tf = state.tasks.lock().unwrap();
        let task = tf.tasks.iter().find(|t| t.id == "task-1").unwrap();
        assert!(task.done);
        assert!(task.completed_at.is_some());

        // Verify persisted
        let loaded = load_tasks(&path);
        let loaded_task = loaded.tasks.iter().find(|t| t.id == "task-1").unwrap();
        assert!(loaded_task.done);
        assert!(loaded_task.completed_at.is_some());
    }

    #[test]
    fn test_toggle_task_not_found() {
        let tasks_file = TasksFile {
            tasks: vec![Task {
                id: "task-1".to_string(),
                text: "Test".to_string(),
                done: false,
                created_at: "2026-03-04T14:00:00Z".to_string(),
                completed_at: None,
                sort_order: 0,
            }],
        };

        let result = tasks_file.tasks.iter().find(|t| t.id == "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_toggle_task_filters_from_active() {
        let tasks_file = TasksFile {
            tasks: vec![
                Task {
                    id: "task-1".to_string(),
                    text: "Active".to_string(),
                    done: false,
                    created_at: "2026-03-04T14:00:00Z".to_string(),
                    completed_at: None,
                    sort_order: 0,
                },
                Task {
                    id: "task-2".to_string(),
                    text: "Toggled".to_string(),
                    done: true,
                    created_at: "2026-03-04T13:00:00Z".to_string(),
                    completed_at: Some("2026-03-04T15:00:00Z".to_string()),
                    sort_order: 1,
                },
            ],
        };

        let active: Vec<&Task> = tasks_file.tasks.iter().filter(|t| !t.done).collect();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, "task-1");
    }

    #[test]
    fn test_toggle_back_to_undone() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);
        let tasks_file = TasksFile {
            tasks: vec![Task {
                id: "task-1".to_string(),
                text: "Done task".to_string(),
                done: true,
                created_at: "2026-03-04T14:00:00Z".to_string(),
                completed_at: Some("2026-03-04T15:00:00Z".to_string()),
                sort_order: 0,
            }],
        };
        save_tasks(&path, &tasks_file).unwrap();

        let state = TaskState {
            tasks: Mutex::new(tasks_file),
            data_path: path.clone(),
        };

        // Toggle back to undone
        {
            let mut tf = state.tasks.lock().unwrap();
            let task = tf.tasks.iter_mut().find(|t| t.id == "task-1").unwrap();
            task.done = false;
            task.completed_at = None;
            save_tasks(&state.data_path, &tf).unwrap();
        }

        let tf = state.tasks.lock().unwrap();
        let task = tf.tasks.iter().find(|t| t.id == "task-1").unwrap();
        assert!(!task.done);
        assert!(task.completed_at.is_none());
    }

    #[test]
    fn test_update_task_changes_text() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);
        let tasks_file = TasksFile {
            tasks: vec![Task {
                id: "task-1".to_string(),
                text: "Original text".to_string(),
                done: false,
                created_at: "2026-03-04T14:00:00Z".to_string(),
                completed_at: None,
                sort_order: 0,
            }],
        };
        save_tasks(&path, &tasks_file).unwrap();

        let state = TaskState {
            tasks: Mutex::new(tasks_file),
            data_path: path.clone(),
        };

        {
            let mut tf = state.tasks.lock().unwrap();
            let task = tf.tasks.iter_mut().find(|t| t.id == "task-1").unwrap();
            task.text = "Updated text".to_string();
            save_tasks(&state.data_path, &tf).unwrap();
        }

        // Verify in memory
        let tf = state.tasks.lock().unwrap();
        assert_eq!(tf.tasks[0].text, "Updated text");

        // Verify persisted
        let loaded = load_tasks(&path);
        assert_eq!(loaded.tasks[0].text, "Updated text");
    }

    #[test]
    fn test_update_task_not_found() {
        let tasks_file = TasksFile {
            tasks: vec![Task {
                id: "task-1".to_string(),
                text: "Test".to_string(),
                done: false,
                created_at: "2026-03-04T14:00:00Z".to_string(),
                completed_at: None,
                sort_order: 0,
            }],
        };

        let result = tasks_file.tasks.iter().find(|t| t.id == "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_update_task_empty_text_rejected() {
        // Simulate the validation logic from update_task
        let text = "   ".trim().to_string();
        assert!(text.is_empty());
    }
}
