use crate::db_conn::db_manager::add_task;
use std::io;

pub async fn get_task_info() -> Option<u64> {
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
    
    println!("g");
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
