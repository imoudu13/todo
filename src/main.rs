mod utils;
mod task_manager;
mod db_conn;

use std::io;
use utils::menu_options::list_options;
use task_manager::tasks::add_task_info;
use task_manager::tasks::print_tasks;
use task_manager::tasks::delete_task_from_table;
use task_manager::tasks::complete_task;

#[tokio::main]
async fn main() {
    let mut choice: String = String::new();
    
    loop {
        choice.clear(); // Remember to clear the variable if it's being reused
        list_options();
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        let trimmed: &str = choice.trim();
        
        if trimmed == "0" {
            break;
        }
        
        match trimmed {
            "1" => {
                add_task_info().await;
            },
            "2" => {
                print_tasks(true).await;
            },
            "3" => { 
                delete_task_from_table().await;
            },
            "4" => {
                complete_task().await;
            },
            _ => println!("Not a valid input"),
        }
    }
}
