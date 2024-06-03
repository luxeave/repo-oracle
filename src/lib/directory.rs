use std::fs;
use std::path::Path;
use crate::gitignore::should_exclude;

// Function to create a textual representation of the directory structure
pub fn create_directory_structure(dir: &Path, exclude_patterns: &[String], indent: &str) -> String {
    let mut output = String::new();
    let files: Vec<_> = match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|file| {
                let should_exclude = should_exclude(&file.path(), exclude_patterns);
                println!("File: {:?}, Excluded: {}", file.path(), should_exclude);
                !should_exclude
            })
            .collect(),
        Err(e) => {
            println!("Error reading directory: {:?}, Error: {}", dir, e);
            Vec::new()
        },
    };

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