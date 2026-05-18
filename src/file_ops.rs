use std::fs;
use std::fs::FileType;
use std::io;

pub fn create_file(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: create <file>");
        return;
    }

    match fs::write(args[1], "") {
        Ok(_) => println!("File created"),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn write_file(args: Vec<&str>) {
    if args.len() < 3 {
        println!("Usage: write <file> <content>");
        return;
    }

    match fs::write(args[1], args[2]) {
        Ok(_) => println!("Written successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn read_file(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: read <file>");
        return;
    }

    match fs::read_to_string(args[1]) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn delete_file(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: delete <file>");
        return;
    }

    match fs::remove_file(args[1]) {
        Ok(_) => println!("Deleted"),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn get_file_type(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: file_type <file>");
        return;
    }

    match fs::metadata(args[1]) {
        Ok(metadata) => {
            println!("{:?}", metadata.file_type());
        }
        Err(err) => println!("Error figuring out the file type : {}", err),
    };
}
