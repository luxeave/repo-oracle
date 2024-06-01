use std::env;
use std::fs;
use std::path::Path;
use std::process;
use oracle::gitignore::read_gitignore;
use oracle::directory::create_directory_structure;
use oracle::extract::extract_definitions;
use oracle::db::process_files;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: directory_structure <command> [options]");
        println!("Commands:");
        println!("  dir    Generate the directory structure");
        println!("  extract    Extract definitions from files");
        process::exit(1);
    }

    let command = &args[1];

    if command == "dir" {
        let mut path_flag_index = None;
        for (i, arg) in args.iter().enumerate() {
            if arg == "--path" {
                path_flag_index = Some(i);
                break;
            }
        }

        let root_dir_path = if let Some(index) = path_flag_index {
            if index + 1 < args.len() {
                args[index + 1].clone()
            } else {
                println!("Missing value for --path flag.");
                process::exit(1);
            }
        } else {
            env::current_dir().unwrap().to_str().unwrap().to_string()
        };

        let root_dir = Path::new(&root_dir_path);

        // Read the .gitignore file and get the exclude patterns
        let exclude_patterns = read_gitignore(&root_dir);

        // Create textual representation of the directory structure
        let directory_structure = create_directory_structure(&root_dir, &exclude_patterns, "");
        println!("Directory Structure:\n{}", directory_structure);

        // Save the directory structure to a file
        let output_file = root_dir.join("directory_structure.txt");
        fs::write(output_file, directory_structure).unwrap();
    } else if command == "extract" {
        let mut path_flag_index = None;
        let mut ext_flag_index = None;
        for (i, arg) in args.iter().enumerate() {
            if arg == "--path" {
                path_flag_index = Some(i);
            } else if arg == "--ext" {
                ext_flag_index = Some(i);
            }
        }

        let root_dir_path = if let Some(index) = path_flag_index {
            if index + 1 < args.len() {
                args[index + 1].clone()
            } else {
                println!("Missing value for --path flag.");
                process::exit(1);
            }
        } else {
            env::current_dir().unwrap().to_str().unwrap().to_string()
        };

        let file_extension = if let Some(index) = ext_flag_index {
            if index + 1 < args.len() {
                args[index + 1].clone()
            } else {
                println!("Missing value for --ext flag.");
                process::exit(1);
            }
        } else {
            "js".to_string()
        };

        let root_dir = Path::new(&root_dir_path);
        let output_file = root_dir.join("extracted.txt");

        extract_definitions(&root_dir, &file_extension, &output_file);
    } else if command == "db" {
        let mut path_flag_index = None;
        for (i, arg) in args.iter().enumerate() {
            if arg == "--path" {
                path_flag_index = Some(i);
                break;
            }
        }

        let root_dir_path = if let Some(index) = path_flag_index {
            if index + 1 < args.len() {
                args[index + 1].clone()
            } else {
                println!("Missing value for --path flag.");
                process::exit(1);
            }
        } else {
            env::current_dir().unwrap().to_str().unwrap().to_string()
        };

        let root_dir = Path::new(&root_dir_path);
        process_files(&root_dir);
    } else {
        println!("Invalid command. Available commands: dir, extract, db");
        process::exit(1);
    }
}