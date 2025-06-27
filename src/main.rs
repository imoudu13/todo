mod utils;
mod task_manager;

use std::io;
use utils::menu_options::list_options;

fn main() {
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
            "1" => println!("We're adding a task eh."),
            "2" => println!("We're listing tasks eh."),
            "3" => println!("We're removing a task eh."),
            "4" => println!("We're marking a task as done eh."),
            _ => println!("Not a valid input"),
        }
    }
}
