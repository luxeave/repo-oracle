// src/lib/gitignore.rs
use glob::Pattern;
use std::fs;
use std::path::Path;

pub fn read_gitignore(dir: &Path) -> Vec<String> {
    let gitignore_path = dir.join(".gitignore");
    if gitignore_path.exists() {
        println!("Found .gitignore file at: {:?}", gitignore_path);
        let contents = fs::read_to_string(gitignore_path).unwrap_or_default();
        let patterns = contents
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect();
        println!("Exclude patterns: {:?}", patterns);
        patterns
    } else {
        println!("No .gitignore file found in: {:?}", dir);
        Vec::new()
    }
}

pub fn should_exclude(path: &Path, exclude_patterns: &[String]) -> bool {
    let relative_path = path.to_str().unwrap_or_default();

    if relative_path.contains("/.git/") || relative_path.contains("/target/") {
        return true;
    }

    exclude_patterns.iter().any(|pattern| {
        let mut full_pattern = if pattern.starts_with('/') {
            pattern.to_string()
        } else {
            format!("**/{}", pattern)
        };

        if !full_pattern.ends_with('/') && !full_pattern.ends_with("**") {
            full_pattern.push_str("/**");
        }

        Pattern::new(&full_pattern).unwrap().matches(relative_path)
    })
}
