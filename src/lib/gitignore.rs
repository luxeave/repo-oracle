use std::fs;
use std::path::Path;
use glob::Pattern;

pub fn read_gitignore(dir: &Path) -> Vec<String> {
    let gitignore_path = dir.join(".gitignore");
    if gitignore_path.exists() {
        println!("Found .gitignore file at: {:?}", gitignore_path);
        let contents = fs::read_to_string(gitignore_path).unwrap_or_default();
        let patterns = contents.lines().map(|line| line.trim().to_string()).collect();
        println!("Exclude patterns: {:?}", patterns);
        patterns
    } else {
        println!("No .gitignore file found in: {:?}", dir);
        Vec::new()
    }
}

pub fn should_exclude(path: &Path, exclude_patterns: &[String]) -> bool {
    let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
    let dir_name = match path.parent() {
        Some(parent) => parent.file_name().unwrap_or_default().to_str().unwrap_or_default(),
        None => "",
    };

    if file_name.starts_with(".git") || file_name == "target" || dir_name == "target" {
        return true;
    }

    exclude_patterns.iter().any(|pattern| {
        let pattern = if pattern.starts_with('/') {
            &pattern[1..]
        } else {
            pattern
        };
        let is_directory = pattern.ends_with('/');
        let pattern = if is_directory {
            format!("{}**", pattern)
        } else {
            pattern.to_string()
        };
        Pattern::new(&pattern).unwrap().matches(file_name) || Pattern::new(&pattern).unwrap().matches(dir_name)
    })
}