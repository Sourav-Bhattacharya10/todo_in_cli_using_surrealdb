use std::{fs, mem};
use serde_json;
use easy_sgr_macros::sgr;

use super::todo::Todo;

#[derive(Debug)]
pub struct TodoList {
    pub path: String,
    contents: String,
    pub todos: Vec<Todo>
}

impl TodoList {
    fn write_to_file_back(&self) {
        let stringified_todos = serde_json::to_string_pretty(&self.todos).unwrap();

        fs::write(&self.path, stringified_todos).expect("Failed to create file");
    }

    pub fn parse_contents_to_vec_todos(&mut self) {
        self.todos = serde_json::from_str(self.contents.as_str()).unwrap();
    }

    pub fn add_todos(&mut self, parameters: Vec<String>) {
        for param in parameters {
            let new_todo = Todo {
                task_name: param.to_string(),
                done_status: false
            };

            self.todos.push(new_todo);
        }

        self.write_to_file_back();
    }

    pub fn display_todos(&self) {
        println!("Todos List:");
        for (i,todo) in self.todos.iter().enumerate() {
            if todo.done_status {
                let striked_task = sgr!("{[strike]}").to_owned() + todo.task_name.as_str() + sgr!("{[]}");
                println!("{} {}", i, striked_task);
            }
            else {
                println!("{} {}", i, todo.task_name);
            }
        }
    }

    pub fn mark_todos_done(&mut self, parameters: Vec<String>) {
        for param in parameters {
            let param_str = param.as_str();
            let param_index = param_str.parse::<usize>().unwrap();
            let _ = mem::replace(&mut self.todos[param_index].done_status, true);
        }

        self.write_to_file_back();
    }

    pub fn mark_todos_undone(&mut self, parameters: Vec<String>) {
        for param in parameters {
            let param_str = param.as_str();
            let param_index = param_str.parse::<usize>().unwrap();
            let _ = mem::replace(&mut self.todos[param_index].done_status, false);
        }

        self.write_to_file_back();
    }

    pub fn remove_todos(&mut self, parameters: Vec<String>) {
        for param in parameters {
            let param_str = param.as_str();
            let param_index = param_str.parse::<usize>().unwrap();
            self.todos.remove(param_index);
        }

        self.write_to_file_back();
    }
}

pub fn create_or_load_instance() -> TodoList {
    let json_file_path = env!("JSON_FILE_PATH");

    let mut todo_list = TodoList {
        path: json_file_path.to_string(),
        contents: String::from(""),
        todos: Vec::new()
    };

    let contents = fs::read_to_string(json_file_path).unwrap_or("".to_string());

    if contents == "" {
        match fs::File::open(json_file_path) {
            Ok(_file) => println!("{json_file_path} file is present but no content"),
            Err(_err) => {
                eprintln!("No {} file found. Creating one now..", json_file_path);
                fs::write(json_file_path, "").expect("Failed to create file");
            }
        }
    }
    else {
        todo_list.contents = contents;
        todo_list.parse_contents_to_vec_todos();
    }

    todo_list
}