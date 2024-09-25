// Define the TaskStatus enum
#[derive(Debug)]
pub enum TaskStatus {
    InProgress,
    Todo,
    Done,
}

// Define the Task struct
#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub task_name: String,
    pub task_state: TaskStatus,
}
// Implement associated functions for Task
impl Task {
    // Constructor for Task
    pub fn new(id: u32, task_name: String, task_state: TaskStatus) -> Task {
        Task {
            id,
            task_name,
            task_state,
        }
    }

    // Method to display task details
    pub fn display(&self) {
        println!(
            "Task ID: {}, Task Name: {}, Task State: {:?}",
            self.id, self.task_name, self.task_state
        );
    }

    // You can add more methods as needed
}