mod stack;

use stack::Stack;
use std::io::{ self, Write};

fn main() {
    loop {
        println!("Select a data structure:\n");
        println!("1. Stack\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => stack_interface(),
            2 => println!("Linked List selected (not yet implemented)"),
            3 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn stack_interface() {
    let mut stack = Stack::new();

    loop {
        println!("Select an operation:\n");
        println!("1. Push\n");
        println!("2. Pop\n");
        println!("3. Peek\n");
        println!("4. Check if empty\n");
        println!("5. Exit");

        let mut operation = String::new();
        io::stdin().read_line(&mut operation ).expect("Failed to read line");
        let operation: u32 = match operation.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    };
}