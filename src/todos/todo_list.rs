use easy_sgr_macros::sgr;
use surrealdb::{engine::remote::ws::Client, opt::PatchOp, Surreal};

use super::todo::Todo;

#[derive(Debug)]
pub struct TodoList<'a> {
    db: Surreal<Client>,
    table_name: &'a str,
    pub todos: Vec<Todo>,
}

impl<'a> TodoList<'a> {
    pub async fn add_todos(&mut self, parameters: Vec<String>) -> surrealdb::Result<()> {
        let mut new_todos: Vec<Todo> = Vec::new();
        for param in parameters {
            let new_todo = Todo {
                task_id: Option::None,
                task_name: param.to_string(),
                done_status: false
            };

            new_todos.push(new_todo);
        }

        // Create a new todo with a random id
        let created: Vec<Todo> = self.db
            .insert(self.table_name)
            .content(new_todos).await?;

        println!("{:?}", created);

        Ok(())
    }

    pub async fn display_todos(&mut self) -> surrealdb::Result<()> {
        println!("Todos List:");
        self.todos = self.db.select(self.table_name).await?;

        for (i,todo) in self.todos.iter().enumerate() {
            if todo.done_status {
                let striked_task = sgr!("{[strike]}").to_owned() + todo.task_name.as_str() + sgr!("{[]}");
                println!("{} {} id: {}", i, striked_task, todo.task_id.as_ref().unwrap());
            }
            else {
                println!("{} {} id: {}", i, todo.task_name, todo.task_id.as_ref().unwrap());
            }
        }

        Ok(())
    }

    pub async fn mark_todos_done(&mut self, parameters: Vec<String>) -> surrealdb::Result<()> {
        for param in parameters {
            let param_parts: Vec<&str> = param.split(":").collect();

            let selected: Option<Todo> = self.db.select((param_parts[0], param_parts[1])).await?;

            let mut _updated: Option<Todo> = self.db.update((param_parts[0], param_parts[1])).content(Todo {
                task_id: selected.clone().unwrap().task_id,
                task_name: selected.clone().unwrap().task_name,
                done_status: true
            }).await?;
        }

        Ok(())
    }

    pub async fn mark_todos_undone(&mut self, parameters: Vec<String>) -> surrealdb::Result<()> {
        for param in parameters {
            let param_parts: Vec<&str> = param.split(":").collect();

            // update can be done using pathc operation as well
            let _updated: Option<Todo> = self.db.update((param_parts[0], param_parts[1]))
                .patch(PatchOp::replace("/doneStatus", false))
                .await?;
        }

        Ok(())
    }

    pub async fn remove_todos(&mut self, parameters: Vec<String>) -> surrealdb::Result<()> {
        for param in parameters {
            let param_parts: Vec<&str> = param.split(":").collect();
            let _deleted: Option<Todo> = self.db.delete((param_parts[0], param_parts[1])).await?;
        }

        Ok(())
    }
}

pub fn create_or_load_instance(db: Surreal<Client>) -> TodoList<'static> {
    let todo_list = TodoList { db, table_name: env!("SURREALDB_TABLENAME"), todos: Vec::new() };

    todo_list
}