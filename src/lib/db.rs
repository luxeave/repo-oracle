use std::fs;
use std::path::Path;
use rusqlite::{Connection, params};
use sha2::{Sha256, Digest};
use crate::gitignore::{read_gitignore, should_exclude};

pub fn process_files(root_dir: &Path) {
    let db_path = root_dir.join("context.db");
    let conn = Connection::open(&db_path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_path VARCHAR,
            content TEXT,
            checksum_prev VARCHAR DEFAULT NULL,
            checksum VARCHAR DEFAULT NULL,
            summary TEXT DEFAULT NULL
        )",
        [],
    ).unwrap();

    let mut file_paths = Vec::new();
    let exclude_patterns = read_gitignore(&root_dir);
    traverse_directory(root_dir, &mut file_paths, &conn, &exclude_patterns);

    // Delete entries from the "files" table that are no longer present in the file system
    let placeholders = std::iter::repeat("?").take(file_paths.len()).collect::<Vec<_>>().join(",");
    let sql = format!("DELETE FROM files WHERE file_path NOT IN ({})", placeholders);
    let params: Vec<&dyn rusqlite::ToSql> = file_paths.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
    conn.execute(&sql, params.as_slice()).unwrap();
}

fn traverse_directory(dir: &Path, file_paths: &mut Vec<String>, conn: &Connection, exclude_patterns: &[String]) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if should_exclude(&path, exclude_patterns) {
            continue;
        }

        if path.is_file() {
            let file_path = path.to_str().unwrap().to_string();
            file_paths.push(file_path.clone());

            let file_content = fs::read(&path).unwrap();
            let file_content = String::from_utf8_lossy(&file_content);
            let checksum = calculate_checksum(&file_content);

            let mut stmt = conn.prepare("SELECT * FROM files WHERE file_path = ?").unwrap();
            let mut rows = stmt.query(&[&file_path]).unwrap();

            if let Some(row) = rows.next().unwrap() {
                let id: i64 = row.get(0).unwrap();
                let checksum_prev: String = row.get(4).unwrap();

                conn.execute(
                    "UPDATE files SET checksum_prev = ?, checksum = ?, content = ? WHERE id = ?",
                    params![checksum_prev, checksum, file_content, id],
                ).unwrap();
            } else {
                conn.execute(
                    "INSERT INTO files (file_path, content, checksum) VALUES (?, ?, ?)",
                    params![file_path, file_content, checksum],
                ).unwrap();
            }
        } else if path.is_dir() {
            traverse_directory(&path, file_paths, conn, exclude_patterns);
        }
    }
}

fn calculate_checksum(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}