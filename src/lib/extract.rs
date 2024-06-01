use std::fs;
use std::io::Write;
use std::path::Path;
use regex::Regex;
use crate::gitignore::{read_gitignore, should_exclude};

pub fn extract_definitions(dir: &Path, file_extension: &str, output_file: &Path) {
    let mut extracted_content = String::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == file_extension {
            let file_content = fs::read_to_string(&path).unwrap();
            let extracted_definitions = extract_definitions_from_file(&path, &file_content);
            extracted_content.push_str(&extracted_definitions);
            extracted_content.push('\n');
        } else if path.is_dir() {
            extract_definitions(&path, file_extension, output_file);
        }
    }

    if !extracted_content.is_empty() {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file)
            .unwrap();
        file.write_all(extracted_content.as_bytes()).unwrap();
    }
}

fn extract_definitions_from_file(file_path: &Path, file_content: &str) -> String {
    let class_regex = Regex::new(r"class\s+(\w+)(\s+extends\s+\w+)?\s*\{[^}]*\}").unwrap();
    let method_regex = Regex::new(r"(constructor\s*\([^)]*\)|\w+\s*\([^)]*\))").unwrap();
    let function_regex = Regex::new(r"function\s+(\w+)\s*\([^)]*\)\s*\{").unwrap();
    let require_regex = Regex::new("const\\s+(\\w+)\\s*=\\s*require\\(\\s*['\"]([^'\"]+)['\"]\\s*\\)").unwrap();
    let arrow_function_regex = Regex::new(r"(\w+)\s*=\s*\([^)]*\)\s*=>\s*\{").unwrap();
    let import_export_regex = Regex::new(r"\b(import|export)\b.*;").unwrap();

    let mut result = String::new();

    // Add a header with the file name
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    result.push_str(&format!("// --------- {} ---------\n", file_name));

    // Extract class declarations and methods
    for class_match in class_regex.captures_iter(file_content) {
        let class_declaration = class_match.get(0).unwrap().as_str();
        let class_name = class_match.get(1).unwrap().as_str();
        let extends_clause = class_match.get(2).map(|m| m.as_str()).unwrap_or("");
        result.push_str(&format!("class {}{} {{\n", class_name, extends_clause));

        for method_match in method_regex.captures_iter(class_declaration) {
            result.push_str(&format!("  {}\n", method_match.get(0).unwrap().as_str()));
        }

        result.push_str("}\n");
    }

    // Extract function declarations
    for function_match in function_regex.captures_iter(file_content) {
        result.push_str(&format!("function {}()\n", function_match.get(1).unwrap().as_str()));
    }

    // Extract const declarations initialized with require
    for require_match in require_regex.captures_iter(file_content) {
        let variable_name = require_match.get(1).unwrap().as_str();
        let module_name = require_match.get(2).unwrap().as_str();
        result.push_str(&format!("const {} = require('{}')\n", variable_name, module_name));
    }

    // Extract arrow functions
    for arrow_function_match in arrow_function_regex.captures_iter(file_content) {
        result.push_str(&format!("{} = () => {{}}\n", arrow_function_match.get(1).unwrap().as_str()));
    }

    // Extract import/export statements
    for import_export_match in import_export_regex.captures_iter(file_content) {
        result.push_str(&format!("{}\n", import_export_match.get(0).unwrap().as_str()));
    }

    result
}

// lib/extract.rs
// ...

pub fn raw_content(dir: &Path, file_extensions: &[String], output_file: &Path) {
    let mut raw_content = String::new();

    // Read the .gitignore file and get the exclude patterns
    let exclude_patterns = read_gitignore(&dir);

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if the file or directory should be excluded
        if should_exclude(&path, &exclude_patterns) {
            continue;
        }

        if path.is_file() && file_extensions.iter().any(|ext| path.extension().unwrap_or_default().to_str().unwrap() == ext) {
            let file_content = fs::read_to_string(&path).unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            raw_content.push_str(&format!("// --------- {} ---------\n", file_name));
            raw_content.push_str(&file_content);
            raw_content.push('\n');
        } else if path.is_dir() {
            self::raw_content(&path, file_extensions, output_file);
        }
    }

    if !raw_content.is_empty() {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file)
            .unwrap();
        file.write_all(raw_content.as_bytes()).unwrap();
    }
}

// ...