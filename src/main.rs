
// use std::env;
use serde::{Deserialize, Serialize};
use tokio;
use surrealdb::{self, engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

// use todo_in_cli_using_surrealdb::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u16
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // let args = env::args().collect();
    // let config_object = Config::build(&args).unwrap();
    // todo_in_cli_using_surrealdb::run(config_object);

    // Connect to the server
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root"
    }).await?;

    // Select a specific namespace / database
    db.use_ns("SurrealDB").use_db("mydatabase").await?;

    // Select all people records
    let persons: Vec<Person> = db.select("Person").await?;
    // for person in persons {

    // }
    println!("{:?}", persons);

    Ok(())
}


// async fn say_world() {
//     println!("world");
// }

// #[tokio::main]
// async fn main() {
//     // Calling `say_world()` does not execute the body of `say_world()`.
//     let op = say_world();

//     // This println! comes first
//     println!("hello");

//     // Calling `.await` on `op` starts executing `say_world`.
//     op.await;
// }