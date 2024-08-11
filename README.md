# TODO_IN_CLI
All the todos will be stored in todos table inside mydatabase.db

Run the surrealdb server first:
surreal start --auth file:mydatabase.db

By default, surrealdb server will be running in localhost:8000.
The default user and password is root

You can change config values in .cargo/config.toml file

Following commands are available to run the application locally:
* cargo run -- list
* cargo run -- add "Buy Banana" "Buy Apple"
* cargo run -- done todos:2yl8sdxyhnwrlnwvljhy todos:sol8kzrrkzdghpc0tvfr
* cargo run -- undone todos:2yl8sdxyhnwrlnwvljhy todos:sol8kzrrkzdghpc0tvfr
* cargo run -- remove todos:2yl8sdxyhnwrlnwvljhy todos:sol8kzrrkzdghpc0tvfr