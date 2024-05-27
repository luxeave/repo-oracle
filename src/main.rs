use std::env;
use std::fs;
use std::path::Path;
use std::process;
use oracle::gitignore::read_gitignore;
use oracle::directory::create_directory_structure;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: directory_structure dir [--path <path_to_root_dir>]");
        process::exit(1);
    }

    let command = &args[1];
    if command != "dir" {
        println!("Invalid command. Use 'dir' to generate the directory structure.");
        process::exit(1);
    }

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
}