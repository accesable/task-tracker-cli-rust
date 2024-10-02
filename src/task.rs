use std::fmt;

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
    pub fn serialize_task(&self) -> String{
        format!(
                    "{{\"id\": {}, \"title\": \"{}\", \"completed\": \"{}\"}},\r\n",
        self.id, self.task_name, self.task_state
        )
    }
}

// Implementing Display for TaskStatus as well
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Todo => "To Do",
            TaskStatus::Done => "Done",
        };
        write!(f, "{}", status_str)
    }
}
