use std::env;
fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Print all the arguments
    println!("Command-line arguments: {:?}", args);

    // Check if there are any arguments
    if args.len() > 2 {
        // Access individual arguments
        println!("Argument 1: {}", args[1]);
        resolving_args(args);
    } else {
        println!("No arguments provided.");
    }
}

fn resolving_args(args : Vec<String>) -> (String, u32, String) {
    let operation : String = args[1].clone();
    let task_id : u32 = 0;
    let mut task_name : String = String::new();

    if operation == "add" || operation == "list" {
        task_name = args[2].clone();
        println!("new task added : {}",task_name);
    }
    (operation,task_id,task_name)
}
