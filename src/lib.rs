mod todos;

use todos::todo_list;

#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub parameters: Vec<String>
}

impl Config {
    pub fn build(args: & Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return  Err("Expected at least one CLI argument. Please run \"cargo run -- help\" to get more info");
        }

        let command = args[1].clone();
        let parameters: Vec<String> = args[2..].to_vec();
        

        Ok(Config { command, parameters })
    }
}

pub fn run(config: Config) {
    let mut todo_list_object = todo_list::create_or_load_instance();

    match config.command.as_str() {
        "help" => helper(),
        "list" => todo_list_object.display_todos(),
        "add" => {
            println!("adding {:?}", config.parameters);
            todo_list_object.add_todos(config.parameters);
        },
        "done" => {
            println!("marking following todos as done: \n{:?}", config.parameters);
            todo_list_object.mark_todos_done(config.parameters);
        },
        "undone" => {
            println!("marking following todos as undone: \n{:?}", config.parameters);
            todo_list_object.mark_todos_undone(config.parameters);
        },
        "remove" => {
            println!("removing following todos: \n{:?}", config.parameters);
            todo_list_object.remove_todos(config.parameters);
        },
        &_ => println!("invalid command. Please run \"cargo run -- help\" to get more info")
    }
}

pub fn helper() {
    println!("Following commands are available to run locally:");
    println!("cargo run -- list");
    println!("cargo run -- add \"Buy Banana\" \"Buy Apple\"");
    println!("cargo run -- done 1 2");
    println!("cargo run -- undone 1 2");
    println!("cargo run -- remove 2 3");
}

