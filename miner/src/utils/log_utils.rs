use crate::error::Result;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn read_log_file(path: &PathBuf, lines: Option<usize>) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    
    if let Some(line_count) = lines {
        let lines: Vec<&str> = content.lines().collect();
        let start = if lines.len() > line_count {
            lines.len() - line_count
        } else {
            0
        };
        content = lines[start..].join("\n");
    }
    
    Ok(content)
}

pub fn get_operator_logs(lines: Option<usize>) -> Result<String> {
    let path = crate::config::get_paths()?.operator_log_path.clone();
    read_log_file(&path, lines)
}

pub fn get_user_logs(lines: Option<usize>) -> Result<String> {
    let path = crate::config::get_paths()?.user_log_path.clone();
    read_log_file(&path, lines)
}