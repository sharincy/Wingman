use std::io;

// Define a custom data structure for tasks
struct Task {
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    // Constructor
    fn new(title: String, description: String) -> Task {
        Task {
            title,
            description,
            completed: false,
        }
    }

    // A method to convert a task to a string for saving to a file
    fn to_string(&self) -> String {
        format!("Title: {}\nDescription: {}\nCompleted: {}\n", self.title, self.description, self.completed)
    }

    // A method to create a task from a string loaded from a file
    fn from_string(s: &str) -> Task {
        let mut title = String::new();
        let mut description = String::new();
        let mut completed = false;

        for line in s.lines() {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                match parts[0] {
                    "Title" => title = parts[1].to_string(),
                    "Description" => description = parts[1].to_string(),
                    "Completed" => completed = parts[1].trim() == "true",
                    _ => {}
                }
            }
        }

        Task {
            title,
            description,
            completed,
        }
    }
}

fn main_menu(tasks: &mut Vec<Task>) {
    println!("Todo List Menu:");
    println!("1. View Tasks");
    println!("2. Add Task");
    println!("3. Edit Task");
    println!("4. Delete Task");
    println!("5. Mark as Complete");
    println!("0. Exit");

    let choice = get_user_input();

    match choice.as_str() {
        "1" => view_tasks(tasks),
        "2" => add_task(tasks),
        "3" => edit_task(tasks),
        "4" => delete_task(tasks),
        "5" => mark_as_complete(tasks),
        "0" => return,
        _ => {
            println!("Invalid choice. Please try again.");
            main_menu(tasks);
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()  
} 

fn view_tasks(tasks: &Vec<Task>) {
    println!("Viewing tasks:");
    for (index, task) in tasks.iter().enumerate() {
        println!("Task {}: Title: {}, Description: {}, Completed: {}", index + 1, task.title, task.description, task.completed);
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Adding a new task:");
    println!("Enter task title:");
    let title = get_user_input();
    println!("Enter task description:");
    let description = get_user_input();
    let new_task = Task::new(title, description);
    tasks.push(new_task);
    println!("Task added successfully.");
}

fn edit_task(tasks: &mut Vec<Task>) {
    println!("Editing a task:");
    println!("Enter the task number to edit:");
    let task_number = get_user_input().parse::<usize>().unwrap();

    if task_number <= tasks.len() {
        println!("Enter the updated task title:");
        let title = get_user_input();
        println!("Enter the updated task description:");
        let description = get_user_input();
        let task = &mut tasks[task_number - 1];
        task.title = title;
        task.description = description;
        println!("Task edited successfully.");
    } else {
        println!("Invalid task number.");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("Deleting a task:");
    println!("Enter the task number to delete:");
    let task_number = get_user_input().parse::<usize>().unwrap();

    if task_number <= tasks.len() {
        tasks.remove(task_number - 1);
        println!("Task deleted successfully.");
    } else {
        println!("Invalid task number.");
    }
}

fn mark_as_complete(tasks: &mut Vec<Task>) {
    println!("Marking a task as complete:");
    println!("Enter the task number to mark as complete:");
    let task_number = get_user_input().parse::<usize>().unwrap();

    if task_number <= tasks.len() {
        let task = &mut tasks[task_number - 1];
        task.completed = true;
        println!("Task marked as complete.");
    } else {
        println!("Invalid task number.");
    }
}

fn save_tasks_to_file(tasks: &Vec<Task>, file_name: &str) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file_name)?;

    for task in tasks.iter() {
        let task_string = task.to_string();
        file.write_all(task_string.as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}

fn load_tasks_from_file(file_name: &str) -> Result<Vec<Task>, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut tasks = Vec::new();
    let mut task_str = String::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if !task_str.is_empty() {
                tasks.push(Task::from_string(&task_str));
                task_str.clear();
            }
        } else {
            task_str.push_str(&line);
            task_str.push_str("\n");
        }
    }

    if !task_str.is_empty() {
        tasks.push(Task::from_string(&task_str));
    }

    Ok(tasks)
}

fn main() {
    let file_name = "tasks.txt";

    let mut tasks = match load_tasks_from_file(file_name) {
        Ok(t) => t,
        Err(_) => Vec::new(),
    };

    main_menu(&mut tasks);

    if let Err(e) = save_tasks_to_file(&tasks, file_name) {
        eprintln!("Error saving tasks: {}", e);
    }
}

