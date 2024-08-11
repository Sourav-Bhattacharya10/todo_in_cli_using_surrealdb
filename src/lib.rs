mod todos;

use surrealdb::{self, engine::remote::ws::Ws, opt::auth::Root, Surreal};

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

pub async fn connect_to_surrealdb() -> surrealdb::Result<todo_list::TodoList<'static>> {
    // Connect to the server
    let surrealdb_server = env!("SURREALDB_SERVER");
    let surrealdb_port = env!("SURREALDB_PORT");
    let surrealdb_user = env!("SURREALDB_USER");
    let surrealdb_password = env!("SURREALDB_PASSWORD");
    let surrealdb_namespace = env!("SURREALDB_NAMESPACE");
    let surrealdb_dbname = env!("SURREALDB_DBNAME");

    let db_conn_str = format!("{}:{}", surrealdb_server, surrealdb_port);
    
    let db = Surreal::new::<Ws>(db_conn_str).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: surrealdb_user,
        password: surrealdb_password
    }).await?;

    // Select a specific namespace / database
    db.use_ns(surrealdb_namespace).use_db(surrealdb_dbname).await?;

    Ok(todo_list::create_or_load_instance(db))
}

pub async fn run(config: Config) -> surrealdb::Result<()> {

    let mut todo_list_object = connect_to_surrealdb().await?;

    match config.command.as_str() {
        "help" => Ok(helper()),
        "list" => {
            todo_list_object.display_todos().await?;
            Ok(())
        },
        "add" => {
            println!("adding {:?}", config.parameters);
            todo_list_object.add_todos(config.parameters).await?;
            Ok(())
        },
        "done" => {
            println!("marking following todos as done: \n{:?}", config.parameters);
            todo_list_object.mark_todos_done(config.parameters).await?;
            Ok(())
        },
        "undone" => {
            println!("marking following todos as undone: \n{:?}", config.parameters);
            todo_list_object.mark_todos_undone(config.parameters).await?;
            Ok(())
        },
        "remove" => {
            println!("removing following todos: \n{:?}", config.parameters);
            todo_list_object.remove_todos(config.parameters).await?;
            Ok(())
        },
        &_ => Ok(println!("invalid command. Please run \"cargo run -- help\" to get more info"))
    }
}

pub fn helper() {
    println!("Following commands are available to run locally:");
    println!("cargo run -- list");
    println!("cargo run -- add \"Buy Banana\" \"Buy Apple\"");
    println!("cargo run -- done todos:2yl8sdxyhnwrlnwvljhy todos:sol8kzrrkzdghpc0tvfr");
    println!("cargo run -- undone todos:2yl8sdxyhnwrlnwvljhy todos:sol8kzrrkzdghpc0tvfr");
    println!("cargo run -- remove todos:2yl8sdxyhnwrlnwvljhy todos:sol8kzrrkzdghpc0tvfr");
}

