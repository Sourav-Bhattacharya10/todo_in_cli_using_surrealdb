
use std::env;
use serde::Deserialize;
use tokio;
use surrealdb::{self, sql::Thing};

use todo_in_cli_using_surrealdb::Config;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let args = env::args().collect();
    let config_object = Config::build(&args).unwrap();
    todo_in_cli_using_surrealdb::run(config_object).await?;

    Ok(())
}