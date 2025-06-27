use crate::db_conn::db_manager::add_task;
use crate::db_conn::db_manager::list_tasks;
use std::io;

pub async fn add_task_info() -> Option<u64> {
    let mut title: String = String::new();
    let mut description: String = String::new();

    println!("Enter title: {}", title);
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Enter description: {}", title);
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    match add_task(title.as_str(), description.as_str()).await {
        Ok(rows_affected) => {
            println!("Added {} rows.", rows_affected);
            Some(rows_affected)
        },
        Err(error) => {
            eprintln!("Failed to add task: {}", error);
            None
        }
    }
}

pub async fn print_tasks() {
    match list_tasks().await {
        Ok(rows) => {
            for row in rows {
                let id: i32 = row.get(0);
                let title: &str = row.get(1);
                let description: &str = row.get(2);

                 println!("Id: {}, Title: {}, Description: {}", id, title, description);
            }
        },
        Err(e) => {
            eprintln!("Error getting the rows {}", e);
        }
    }
}
