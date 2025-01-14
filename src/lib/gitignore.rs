// --------- gitignore.rs ---------
use std::fs;
use std::path::Path;
use glob::Pattern;

/// Read lines from `.gitignore` in `dir`. Return them as raw patterns (filtering out comments/empty).
pub fn read_gitignore(dir: &Path) -> Vec<String> {
    let gitignore_path = dir.join(".gitignore");
    if gitignore_path.exists() {
        println!("Found .gitignore file at: {:?}", gitignore_path);
        let contents = fs::read_to_string(&gitignore_path).unwrap_or_default();
        let patterns = contents
            .lines()
            .map(|s| s.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        println!("Exclude patterns: {:?}", patterns);
        patterns
    } else {
        println!("No .gitignore file found in: {:?}", dir);
        Vec::new()
    }
}

/// Decide if a given `path` should be excluded based on the `.gitignore` patterns.
/// This version tries to mimic standard Git behavior more closely.
pub fn should_exclude(path: &Path, exclude_patterns: &[String]) -> bool {
    // Normalize path: convert backslashes to forward slashes, remove repeated slashes
    let path_str = normalize_path(path);

    // Test each pattern
    for pattern in exclude_patterns {
        // Convert `.gitignore` line to a glob pattern
        let mut glob_pat = pattern.clone();

        // Leading slash => pattern is relative to root: remove the slash
        //   e.g. "/target" => "target"
        //   Then we'll match "target" as a top-level directory with **/ prepended
        if glob_pat.starts_with('/') {
            glob_pat.remove(0); // remove leading slash
        }

        // If the pattern does NOT start with '/', prepend "**/"
        // So "target" becomes "**/target"
        // So "Cargo.lock" becomes "**/Cargo.lock"
        if !pattern.starts_with('/') {
            glob_pat = format!("**/{}", glob_pat);
        }

        // If pattern ends with "/", it means ignore that folder and everything under it
        // e.g. "target/" => "**/target/**"
        if glob_pat.ends_with('/') {
            glob_pat.push_str("**");
        }

        // Now attempt matching
        // e.g. "**/target/**", "**/Cargo.lock"
        if Pattern::new(&glob_pat).map(|p| p.matches(&path_str)).unwrap_or(false) {
            return true;
        }
    }

    false
}

/// Convert path to a normalized forward slash form, e.g. "C:/projects/myproj/target" 
fn normalize_path(path: &Path) -> String {
    let path_str = path.to_string_lossy().replace("\\", "/");
    // Remove any double slashes
    let mut out = String::with_capacity(path_str.len());
    let mut prev_was_slash = false;
    for ch in path_str.chars() {
        if ch == '/' {
            if !prev_was_slash {
                out.push(ch);
            }
            prev_was_slash = true;
        } else {
            out.push(ch);
            prev_was_slash = false;
        }
    }
    out
}