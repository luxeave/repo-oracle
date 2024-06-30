use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn retrieve_files(input_file: &Path, output_file: &Path) -> Result<(), std::io::Error> {
    let file = fs::File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut output = String::new();

    for line in reader.lines() {
        let file_path = line?;
        let file_content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => format!("Error reading file: {}", e),
        };

        output.push_str(&format!("----------------------\n{}\n---------------------\n", file_path));
        output.push_str(&file_content);
        output.push_str("\n");
    }

    fs::write(output_file, output)?;
    Ok(())
}