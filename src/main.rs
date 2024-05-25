use std::env;
use std::fs;
use std::path::Path;
use oracle::gitignore::read_gitignore;
use oracle::directory::create_directory_structure;

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_dir_path = if args.len() > 1 {
        let path = &args[1];
        if path == "." {
            env::current_dir().unwrap().to_str().unwrap().to_string()
        } else {
            path.to_string()
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