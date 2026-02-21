use log::{debug, error};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

/// Appends a string to a file, creating the file if it doesn't exist
///
/// # Arguments
/// * `file_path` - Path to the file
/// * `content` - String content to append
///
/// # Returns
/// * `Result<(), io::Error>` - Ok(()) on success, Err on failure
///
/// # Examples
/// ```
/// use crate::utils::append_string_to_file;
///
/// let result = append_string_to_file("log.txt", "Hello, World!\n").await;
/// match result {
///     Ok(()) => println!("Successfully appended to file"),
///     Err(e) => eprintln!("Failed to append to file: {}", e),
/// }
/// ```
pub async fn append_string_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    let path = file_path.as_ref();
    debug!("Appending {} bytes to file: {:?}", content.len(), path);

    tokio::task::spawn_blocking({
        let path = path.to_owned();
        let content = content.to_owned();
        move || {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;

            file.write_all(content.as_bytes())?;
            file.flush()?;
            Ok(())
        }
    }).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
}

/// Synchronous version of append_string_to_file
///
/// # Arguments
/// * `file_path` - Path to the file
/// * `content` - String content to append
///
/// # Returns
/// * `Result<(), io::Error>` - Ok(()) on success, Err on failure
pub fn append_string_to_file_sync<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    let path = file_path.as_ref();
    debug!("Synchronously appending {} bytes to file: {:?}", content.len(), path);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;
    file.flush()?;

    Ok(())
}

/// Appends a string with a newline to a file
///
/// # Arguments
/// * `file_path` - Path to the file
/// * `content` - String content to append (newline will be added automatically)
///
/// # Returns
/// * `Result<(), io::Error>` - Ok(()) on success, Err on failure
pub async fn append_line_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    append_string_to_file(file_path, &format!("{}\n", content)).await
}

/// Synchronous version of append_line_to_file
///
/// # Arguments
/// * `file_path` - Path to the file
/// * `content` - String content to append (newline will be added automatically)
///
/// # Returns
/// * `Result<(), io::Error>` - Ok(()) on success, Err on failure
pub fn append_line_to_file_sync<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    append_string_to_file_sync(file_path, &format!("{}\n", content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[cfg(test)]
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_append_string_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();

        // Test appending to file
        let result = append_string_to_file(file_path, "Hello, ").await;
        assert!(result.is_ok());

        let result = append_string_to_file(file_path, "World!").await;
        assert!(result.is_ok());

        // Read and verify content
        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_append_string_to_file_sync() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();

        // Test appending to file
        let result = append_string_to_file_sync(file_path, "Sync ");
        assert!(result.is_ok());

        let result = append_string_to_file_sync(file_path, "Test");
        assert!(result.is_ok());

        // Read and verify content
        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "Sync Test");
    }

    #[tokio::test]
    async fn test_append_line_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();

        // Test appending lines
        let result = append_line_to_file(file_path, "Line 1").await;
        assert!(result.is_ok());

        let result = append_line_to_file(file_path, "Line 2").await;
        assert!(result.is_ok());

        // Read and verify content
        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "Line 1\nLine 2\n");
    }
}
