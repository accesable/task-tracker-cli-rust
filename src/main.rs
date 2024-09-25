use std::env;
use std::io::{self, Result};
// Declare the module
mod task;

// Import Task and TaskStatus from the task module
use task::{Task, TaskStatus};
fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Print all the arguments
    println!("Command-line arguments: {:?}", args);

    match resolving_args(args) {
        Ok((operation, task_id, task_name)) => {
            println!("Operation: {}, Task ID: {}, Task Name: {}", operation, task_id, task_name);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}


fn resolving_args(args: Vec<String>) -> Result<(String, u32, String)> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not enough arguments"));
    }

    let operation: String = args[1].clone();
    let mut task_id: u32 = 0;
    let mut task_name: String = String::new();

    if operation == "add" && args.len() == 3 {
        task_name = args[2].clone();
        println!("New task added: {}", task_name);
    } else if operation == "list" {
        if args.len() == 2 {
            println!("List all tasks");
        } else {
            task_name = args[2].clone();

            match task_name.as_str() {
                "done" => println!("List all done tasks"),
                "todo" => println!("List all todo tasks"),
                "in-progress" => println!("List all in-progress tasks"),
                _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid task state")),
            }
        }
    } else {
        // Handle invalid task ID
        task_id = match args[2].parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid task ID: Task ID should be a valid positive number"));
            }
        };
        if operation == "update" {
            task_name = args[3].clone();
        }

        match operation.as_str() {
            "update" => println!("Update id {} with name {}", task_id, task_name),
            "mark-done" => println!("Mark task id {} as done", task_id),
            "mark-in-progress" => println!("Mark task id {} as in-progress", task_id),
            "delete" => println!("Delete task id {}",task_id),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid operation")),
        }
    } 
    Ok((operation, task_id, task_name))
}