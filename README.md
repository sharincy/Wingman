# Wingman
Elementary System Programming Project Wingman

# The introduction
I would like to introduce my project from the third category, C3 Personal Data Management. I considered the name "Wingman" to mean someone who provides support and assistance to a friend in various situations. In the context of a to-do list, this name could suggest that this application is there to support and assist users in managing their tasks, just like a helpful friend or "wingman."

# Overview 
The Rust Task Manager is a command-line application that allows users to manage their to-do list by performing various tasks, including adding, viewing, editing, deleting, marking as complete, and exporting tasks to different formats.



# Feature 
The Rust Task Manager provides the following features: 

Add Task: Users can add a new task by specifying a title and a description for the task. 

View Tasks: Users can view the list of tasks along with their titles, descriptions, and completion status. 

Edit Task: Users can edit an existing task by providing the task number, a new title, and a new description. 

Delete Task: Users can delete a task by specifying its task number. 

Mark as Complete: Users can mark a task as complete by specifying its task number. 

Export Tasks: Users can export the list of tasks to a file in either JSON or YAML format. Users can specify the name of the output file and the format.



# Command 
#‘add’ subcommand
> cargo run -- add --title "Task Title" --description "Task Description"

# ‘view’ subcommand
> cargo run -- view

# ‘edit’ subcommand
> cargo run -- edit --task_number 1 --title "Updated Title" --description "Updated Description"

# ‘delete’ subcommand
> cargo run -- delete --task_number 1

# ‘mark-complete’ subcommand
> cargo run -- mark-complete --task_number 1

# ‘export’ subcommand
(export to JSON)
> cargo run -- export --file_name "tasks" --format json

(export to YAML)
> cargo run -- export --file_name "tasks" --format yaml

# Dependencies
> serde = { version = "1.0", features = ["derive"] }
> serde_json = "1.0"
> serde_yaml = "0.8"
> clap = "2.33"


# Tutorial!
Tutorial or example usage are in the PDF file! Please feel free to unfold the unexplored!
