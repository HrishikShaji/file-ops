mod file_ops;

use std::io::{self, Write};

fn main() {
    println!("Simple Rust File CLI");
    println!("Type 'help' for commands");
    println!("Type 'exit' to quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input == "help" {
            println!("Commands:");
            println!("create <file>");
            println!("write <file> <content>");
            println!("read <file>");
            println!("delete <file>");
            println!("file_type <file>");
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts[0] {
            "create" => file_ops::create_file(parts),
            "write" => file_ops::write_file(parts),
            "read" => file_ops::read_file(parts),
            "delete" => file_ops::delete_file(parts),
            "file_type" => file_ops::get_file_type(parts),

            _ => {
                println!("Unknown command");
            }
        }
    }

    println!("Goodbye!");
}
