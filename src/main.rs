mod utils;
mod task_manager;
mod db_conn;

use std::io;
use utils::menu_options::list_options;
use task_manager::tasks::add_task_info;
use task_manager::tasks::print_tasks;

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
                print_tasks().await;
            },
            "3" => println!("We're removing a task eh."),
            "4" => println!("We're marking a task as done eh."),
            _ => println!("Not a valid input"),
        }
    }
}
