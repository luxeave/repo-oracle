use crate::db::process_files;
use crate::directory::create_directory_structure;
use crate::extract::{extract_definitions, raw_content};
use crate::gitignore::read_gitignore;
use crate::retrieve::retrieve_files;
use crate::utils::{parse_path, parse_path_and_ext};
use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub fn handle_dir_command(args: &[String]) {
    let (root_dir_path, output_dir_path) = match args.iter().position(|arg| arg == "--path") {
        Some(index) if index + 1 < args.len() => {
            let path = &args[index + 1];
            (path.clone(), Path::new(path).to_path_buf())
        }
        _ => {
            let current_dir = env::current_dir().unwrap();
            let current_dir_path = current_dir.to_str().unwrap().to_string();
            (current_dir_path, current_dir)
        }
    };

    let root_dir = Path::new(&root_dir_path);

    // Read the .gitignore file and get the exclude patterns
    let exclude_patterns = read_gitignore(&root_dir);

    // Create textual representation of the directory structure
    let directory_structure = create_directory_structure(&root_dir, &exclude_patterns, "");
    println!("Directory Structure:\n{}", directory_structure);

    // Save the directory structure to a file in the current directory
    let output_file = output_dir_path.join("directory_structure.txt");
    fs::write(output_file, directory_structure).unwrap();
}

pub fn handle_extract_command(args: &[String]) {
    let (root_dir_path, file_extension) = parse_path_and_ext(args);
    let root_dir = Path::new(&root_dir_path);
    let output_file = root_dir.join("extracted.txt");

    extract_definitions(&root_dir, &file_extension, &output_file);
}

pub fn handle_raw_command(args: &[String]) {
    let (root_dir_path, file_extensions_str) = parse_path_and_ext(args);
    let root_dir = Path::new(&root_dir_path);
    let output_file = root_dir.join("raw.txt");

    let process_all = file_extensions_str.to_lowercase() == "all";
    let file_extensions = if process_all {
        Vec::new()
    } else {
        file_extensions_str
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };

    raw_content(&root_dir, &file_extensions, &output_file);
}

pub fn handle_db_command(args: &[String]) {
    let root_dir_path = parse_path(args);
    let root_dir = Path::new(&root_dir_path);
    process_files(&root_dir);
}

pub fn handle_retrieve_command(args: &[String]) {
    let mut file_flag_index = None;
    for (i, arg) in args.iter().enumerate() {
        if arg == "--file" {
            file_flag_index = Some(i);
            break;
        }
    }

    let input_file_path = if let Some(index) = file_flag_index {
        if index + 1 < args.len() {
            args[index + 1].clone()
        } else {
            println!("Missing value for --file flag.");
            process::exit(1);
        }
    } else {
        println!("Missing --file flag.");
        process::exit(1);
    };

    let input_file = Path::new(&input_file_path);
    let output_file = Path::new("retrieve.txt");

    match retrieve_files(input_file, output_file) {
        Ok(_) => println!("Retrieved content saved to retrieve.txt"),
        Err(e) => {
            println!("Error retrieving files: {}", e);
            process::exit(1);
        }
    }
}
