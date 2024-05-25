use std::fs;
use std::path::Path;

pub fn read_gitignore(dir: &Path) -> Vec<String> {
    let gitignore_path = dir.join(".gitignore");
    if gitignore_path.exists() {
        let contents = fs::read_to_string(gitignore_path).unwrap();
        contents.lines().map(|line| line.trim().to_string()).collect()
    } else {
        Vec::new()
    }
}

pub fn should_exclude(path: &Path, exclude_patterns: &[String]) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    file_name == ".git" || file_name == ".gitignore" || exclude_patterns.iter().any(|pattern| path.to_str().unwrap().contains(pattern))
}