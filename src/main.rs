use std::env;
use std::fs;
use std::path::Path;

// Function to create a textual representation of the directory structure
fn create_directory_structure(dir: &Path, indent: &str) -> String {
    let mut output = String::new();
    let files: Vec<_> = fs::read_dir(dir).unwrap().collect();

    for (index, file) in files.iter().enumerate() {
        let file = file.as_ref().unwrap();
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
            output.push_str(&create_directory_structure(&file_path, &sub_indent));
        }
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

    // Create textual representation of the directory structure
    let directory_structure = create_directory_structure(&root_dir, "");
    println!("Directory Structure:\n{}", directory_structure);

    // Save the directory structure to a file
    let output_file = root_dir.join("directory_structure.txt");
    fs::write(output_file, directory_structure).unwrap();
}