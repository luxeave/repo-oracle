use std::env;
use std::fs;
use std::path::Path;

// Function to read the .gitignore file and return a vector of patterns
fn read_gitignore(dir: &Path) -> Vec<String> {
    let gitignore_path = dir.join(".gitignore");
    if gitignore_path.exists() {
        let contents = fs::read_to_string(gitignore_path).unwrap();
        contents.lines().map(|line| line.trim().to_string()).collect()
    } else {
        Vec::new()
    }
}

// Function to check if a file or directory should be excluded
fn should_exclude(path: &Path, exclude_patterns: &[String]) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    file_name == ".git" || file_name == ".gitignore" || exclude_patterns.iter().any(|pattern| path.to_str().unwrap().contains(pattern))
}

// Function to create a textual representation of the directory structure
fn create_directory_structure(dir: &Path, exclude_patterns: &[String], indent: &str) -> String {
    let mut output = String::new();
    let files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|file| !should_exclude(&file.path(), exclude_patterns))
        .collect();

    let mut index = 0;
    for file in &files {
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        let is_last_file = index == files.len() - 1;
        let is_directory = file_path.is_dir();

        output.push_str(&format!(
            "{}{}{}\n",
            indent,
            if is_last_file { "└── " } else { "├── " },
            file_name
        ));

        if is_directory {
            let sub_indent = if is_last_file {
                format!("{}    ", indent)
            } else {
                format!("{}│   ", indent)
            };
            output.push_str(&create_directory_structure(&file_path, exclude_patterns, &sub_indent));
        }

        index += 1;
    }

    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_dir_path = if args.len() > 1 {
        args[1].clone()
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