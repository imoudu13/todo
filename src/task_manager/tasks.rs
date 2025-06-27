use crate::db_conn::db_manager::add_task;
use crate::db_conn::db_manager::list_tasks;
use crate::db_conn::db_manager::remove_task;
use crate::db_conn::db_manager::mark_done;

use std::io;
use std::num::ParseIntError;

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

    let clean_title: &str = title.as_str().trim();
    let clean_description: &str = description.as_str().trim();

    match add_task(clean_title, clean_description).await {
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

pub async fn print_tasks(list_description: bool) {
    match list_tasks().await {
        Ok(rows) => {
            for row in rows {
                let id: i32 = row.get(0);
                let title: &str = row.get(1);
                let description: &str = row.get(2);

                println!("------------------------------");
                println!("Id: {}, Title: {}", id, title);
                
                if list_description {
                    println!("Description: {}", description);
                }
            }
        },
        Err(e) => {
            eprintln!("Error getting the rows {}", e);
        }
    }
}

pub async fn delete_task_from_table() {
    let mut task_id: String = String::new();

    print_tasks(false).await;

    println!("Enter task id: ");
    io::stdin().read_line(&mut task_id).expect("Failed to read line");

    let clean_task_id: Result<i32, ParseIntError> = task_id.as_str().trim().parse::<i32>();

    match clean_task_id {
        Ok(id) => {
            remove_task(id).await.expect("TODO: panic message");
        },
        Err(_) => {
            eprintln!("Error getting the task id");
        }
    }
}

pub async fn complete_task() {
    let mut task_id: String = String::new();

    print_tasks(false).await;

    println!("Enter task id: ");
    io::stdin().read_line(&mut task_id).expect("Failed to read line");

    let clean_task_id: Result<i32, ParseIntError> = task_id.as_str().trim().parse::<i32>();

    match clean_task_id {
        Ok(id) => {
            mark_done(id).await.expect("Mark done failed");
        },
        Err(_) => {
            eprintln!("Error getting the task id");
        }
    }
}
