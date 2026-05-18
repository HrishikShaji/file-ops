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
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts[0] {
            "create" => {
                if parts.len() < 2 {
                    println!("Usage: create <file>");
                    continue;
                }

                match file_ops::create_file(parts[1]) {
                    Ok(_) => println!("File created"),
                    Err(e) => println!("Error: {}", e),
                }
            }

            "write" => {
                if parts.len() < 3 {
                    println!("Usage: write <file> <content>");
                    continue;
                }

                match file_ops::write_file(parts[1], parts[2]) {
                    Ok(_) => println!("Written successfully"),
                    Err(e) => println!("Error: {}", e),
                }
            }

            "read" => {
                if parts.len() < 2 {
                    println!("Usage: read <file>");
                    continue;
                }

                match file_ops::read_file(parts[1]) {
                    Ok(content) => println!("{}", content),
                    Err(e) => println!("Error: {}", e),
                }
            }

            "delete" => {
                if parts.len() < 2 {
                    println!("Usage: delete <file>");
                    continue;
                }

                match file_ops::delete_file(parts[1]) {
                    Ok(_) => println!("Deleted"),
                    Err(e) => println!("Error: {}", e),
                }
            }

            _ => {
                println!("Unknown command");
            }
        }
    }

    println!("Goodbye!");
}
