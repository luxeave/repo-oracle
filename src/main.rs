use oracle::commands::{
    handle_db_command, handle_dir_command, handle_extract_command, handle_raw_command,
    handle_retrieve_command,
};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "dir" => handle_dir_command(&args),
        "extract" => handle_extract_command(&args),
        "raw" => handle_raw_command(&args),
        "db" => handle_db_command(&args),
        "retrieve" => handle_retrieve_command(&args),
        _ => {
            println!("Invalid command. Available commands: dir, extract, raw, db, retrieve");
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: directory_structure <command> [options]");
    println!("Commands:");
    println!("  dir      Generate the directory structure");
    println!("  extract  Extract definitions from files");
    println!("  raw      Extract raw content from files");
    println!("  db       Process files for database");
    println!("  retrieve Retrieve content from specified files");
}
