extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use clap::{App, Arg, SubCommand};
use serde::{Serialize, Deserialize};
use std::io;
use std::fs::File;
use std::io::Write;
use std::io::BufRead;

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    fn new(title: String, description: String) -> Task {
        Task {
            title,
            description,
            completed: false,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "Title: {}\nDescription: {}\nCompleted: {}\n",
            self.title, self.description, self.completed
        )
    }

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

fn view_tasks(tasks: &Vec<Task>) {
    println!("Viewing tasks:");
    for (index, task) in tasks.iter().enumerate() {
        println!(
            "Task {}: Title: {}, Description: {}, Completed: {}",
            index + 1,
            task.title,
            task.description,
            task.completed
        );
    }
}

fn add_task(tasks: &mut Vec<Task>, title: &str, description: &str) {
    let new_task = Task::new(title.to_string(), description.to_string());
    tasks.push(new_task);
    println!("Task added successfully.");
}

fn edit_task(tasks: &mut Vec<Task>, task_number: usize, title: &str, description: &str) {
    if task_number <= tasks.len() {
        let task = &mut tasks[task_number - 1];
        task.title = title.to_string();
        task.description = description.to_string();
        println!("Task edited successfully.");
    } else {
        println!("Invalid task number.");
    }
}

fn delete_task(tasks: &mut Vec<Task>, task_number: usize) {
    if task_number <= tasks.len() {
        tasks.remove(task_number - 1);
        println!("Task deleted successfully.");
    } else {
        println!("Invalid task number.");
    }
}

fn mark_as_complete(tasks: &mut Vec<Task>, task_number: usize) {
    if task_number <= tasks.len() {
        let task = &mut tasks[task_number - 1];
        task.completed = true;
        println!("Task marked as complete.");
    } else {
        println!("Invalid task number.");
    }
}

fn save_tasks_to_file(tasks: &Vec<Task>, file_name: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;

    for task in tasks.iter() {
        let task_string = task.to_string();
        file.write_all(task_string.as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}

fn load_tasks_from_file(file_name: &str) -> Result<Vec<Task>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);

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

fn export_tasks_to_json(tasks: &Vec<Task>, file_name: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(tasks)?;

    let mut file = File::create(file_name)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn export_tasks_to_yaml(tasks: &Vec<Task>, file_name: &str) -> Result<(), std::io::Error> {
    let yaml = serde_yaml::to_string(tasks);
    match yaml {
        Ok(yaml) => {
            let mut file = File::create(file_name)?;
            file.write_all(yaml.as_bytes())?;
            Ok(())
        }
        Err(err) => {
            Err(std::io::Error::new(std::io::ErrorKind::Other, err))
        }
    }
}

fn main() {
    let file_name = "tasks.txt";

    let mut tasks = match load_tasks_from_file(file_name) {
        Ok(t) => t,
        Err(_) => Vec::new(),
    };

    let matches = App::new("Todo List")
        .version("1.0")
        .author("Your Name")
        .about("A simple Rust TODO list manager")
        .subcommand(SubCommand::with_name("add")
            .about("Add a new task")
            .arg(Arg::with_name("title")
                .help("Task title")
                .required(true)
            )
            .arg(Arg::with_name("description")
                .help("Task description")
                .required(true)
                .multiple(true) 
            )
        )
        .subcommand(SubCommand::with_name("view")
            .about("View tasks")
        )
        .subcommand(SubCommand::with_name("edit")
            .about("Edit a task")
            .arg(Arg::with_name("task_number")
                .help("Task number to edit")
                .required(true)
            )
            .arg(Arg::with_name("title")
                .help("Updated task title")
                .required(true)
            )
            .arg(Arg::with_name("description")
                .help("Updated task description")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("delete")
            .about("Delete a task")
            .arg(Arg::with_name("task_number")
                .help("Task number to delete")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("mark-complete")
            .about("Mark a task as complete")
            .arg(Arg::with_name("task_number")
                .help("Task number to mark as complete")
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("export")
            .about("Export tasks to a file")
            .arg(Arg::with_name("file_name")
                .help("Name of the output file")
                .required(true)
            )
            .arg(Arg::with_name("format")
                .help("File format (json, yaml, etc.)")
                .required(true)
            )
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let title = add_matches.value_of("title").unwrap();
            let description = add_matches.values_of("description").unwrap().collect::<Vec<&str>>().join(" ");
            add_task(&mut tasks, title, &description);
        }
        ("view", Some(_)) => {
            view_tasks(&tasks);
        }
        ("edit", Some(edit_matches)) => {
            let task_number = edit_matches.value_of("task_number").unwrap().parse::<usize>().unwrap();
            let title = edit_matches.value_of("title").unwrap();
            let description = edit_matches.value_of("description").unwrap();
            edit_task(&mut tasks, task_number, title, description);
        }
        ("delete", Some(delete_matches)) => {
            let task_number = delete_matches.value_of("task_number").unwrap().parse::<usize>().unwrap();
            delete_task(&mut tasks, task_number);
        }
        ("mark-complete", Some(mark_matches)) => {
            let task_number = mark_matches.value_of("task_number").unwrap().parse::<usize>().unwrap();
            mark_as_complete(&mut tasks, task_number);
        }
        ("export", Some(export_matches)) => {
            let file_name = export_matches.value_of("file_name").unwrap();
            let format = export_matches.value_of("format").unwrap();

            let full_file_name = match format {
                "json" => format!("{}.json", file_name),
                "yaml" => format!("{}.yaml", file_name),
                _ => {
                    println!("Invalid format. Supported formats: json, yaml.");
                    return;
                }
            };

            match format {
                "json" => {
                    if let Err(e) = export_tasks_to_json(&tasks, &full_file_name) {
                        eprintln!("Error exporting tasks to JSON: {}", e);
                    } else {
                        println!("Tasks exported to JSON successfully.");
                    }
                }
                "yaml" => {
                    if let Err(e) = export_tasks_to_yaml(&tasks, &full_file_name) {
                        eprintln!("Error exporting tasks to YAML: {}", e);
                    } else {
                        println!("Tasks exported to YAML successfully.");
                    }
                }
                _ => {
                    println!("Invalid format. Supported formats: json, yaml.");
                }
            }
        }
        _ => {
            println!("Invalid command. Use --help for usage information.");
        }
    }

    if let Err(e) = save_tasks_to_file(&tasks, file_name) {
        eprintln!("Error saving tasks: {}", e);
    }
}
