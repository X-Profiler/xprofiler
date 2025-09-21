//! File system utilities for xprofiler

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Check if a path exists
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Check if a path is a file
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Check if a path is a directory
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Get file size in bytes
pub fn file_size<P: AsRef<Path>>(path: P) -> Result<u64, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Get file modification time as timestamp
pub fn file_mtime<P: AsRef<Path>>(path: P) -> Result<u64, io::Error> {
    let metadata = fs::metadata(path)?;
    let mtime = metadata.modified()?;
    let duration = mtime.duration_since(UNIX_EPOCH)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(duration.as_secs())
}

/// Create directory and all parent directories
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    fs::create_dir_all(path)
}

/// Remove file or directory recursively
pub fn remove_all<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let path = path.as_ref();
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

/// Copy file from source to destination
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64, io::Error> {
    fs::copy(from, to)
}

/// Move/rename file or directory
pub fn move_path<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), io::Error> {
    fs::rename(from, to)
}

/// Read entire file as string
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Read entire file as bytes
pub fn read_to_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, io::Error> {
    fs::read(path)
}

/// Write string to file
pub fn write_string<P: AsRef<Path>>(path: P, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Write bytes to file
pub fn write_bytes<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Append string to file
pub fn append_string<P: AsRef<Path>>(path: P, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(content.as_bytes())
}

/// Append bytes to file
pub fn append_bytes<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(content)
}

/// Read file line by line
pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/// Write lines to file
pub fn write_lines<P: AsRef<Path>>(path: P, lines: &[String]) -> Result<(), io::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    for line in lines {
        writeln!(writer, "{}", line)?;
    }
    
    writer.flush()
}

/// Get file extension
pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}

/// Get file name without extension
pub fn get_stem<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
}

/// Get file name
pub fn get_filename<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

/// Get parent directory
pub fn get_parent<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    path.as_ref().parent().map(|p| p.to_path_buf())
}

/// Join paths
pub fn join_path<P: AsRef<Path>, Q: AsRef<Path>>(base: P, path: Q) -> PathBuf {
    base.as_ref().join(path)
}

/// Normalize path (resolve . and .. components)
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut components = Vec::new();
    
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            _ => components.push(component),
        }
    }
    
    components.iter().collect()
}

/// Get absolute path
pub fn absolute_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, io::Error> {
    path.as_ref().canonicalize()
}

/// Check if path is absolute
pub fn is_absolute<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_absolute()
}

/// Check if path is relative
pub fn is_relative<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_relative()
}

/// List directory contents
pub fn list_dir<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        entries.push(entry.path());
    }
    
    entries.sort();
    Ok(entries)
}

/// List directory contents recursively
pub fn list_dir_recursive<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = Vec::new();
    list_dir_recursive_impl(path.as_ref(), &mut entries)?;
    entries.sort();
    Ok(entries)
}

fn list_dir_recursive_impl(path: &Path, entries: &mut Vec<PathBuf>) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        entries.push(path.clone());
        
        if path.is_dir() {
            list_dir_recursive_impl(&path, entries)?;
        }
    }
    
    Ok(())
}

/// Find files matching a pattern
pub fn find_files<P: AsRef<Path>>(
    root: P,
    pattern: &str,
    recursive: bool,
) -> Result<Vec<PathBuf>, io::Error> {
    let mut matches = Vec::new();
    
    if recursive {
        find_files_recursive_impl(root.as_ref(), pattern, &mut matches)?;
    } else {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(filename) = get_filename(&path) {
                    if filename.contains(pattern) {
                        matches.push(path);
                    }
                }
            }
        }
    }
    
    matches.sort();
    Ok(matches)
}

fn find_files_recursive_impl(
    path: &Path,
    pattern: &str,
    matches: &mut Vec<PathBuf>,
) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        
        if entry_path.is_file() {
            if let Some(filename) = get_filename(&entry_path) {
                if filename.contains(pattern) {
                    matches.push(entry_path);
                }
            }
        } else if entry_path.is_dir() {
            find_files_recursive_impl(&entry_path, pattern, matches)?;
        }
    }
    
    Ok(())
}

/// Calculate directory size recursively
pub fn dir_size<P: AsRef<Path>>(path: P) -> Result<u64, io::Error> {
    let mut total_size = 0;
    dir_size_impl(path.as_ref(), &mut total_size)?;
    Ok(total_size)
}

fn dir_size_impl(path: &Path, total_size: &mut u64) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        
        if entry_path.is_file() {
            *total_size += file_size(&entry_path)?;
        } else if entry_path.is_dir() {
            dir_size_impl(&entry_path, total_size)?;
        }
    }
    
    Ok(())
}

/// Create a temporary file
pub fn create_temp_file(prefix: &str, suffix: &str) -> Result<(File, PathBuf), io::Error> {
    let temp_dir = std::env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    
    let filename = format!("{}{}{}", prefix, timestamp, suffix);
    let temp_path = temp_dir.join(filename);
    
    let file = File::create(&temp_path)?;
    Ok((file, temp_path))
}

/// Create a temporary directory
pub fn create_temp_dir(prefix: &str) -> Result<PathBuf, io::Error> {
    let temp_dir = std::env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    
    let dirname = format!("{}{}", prefix, timestamp);
    let temp_path = temp_dir.join(dirname);
    
    fs::create_dir_all(&temp_path)?;
    Ok(temp_path)
}

/// Safe file operations with backup
pub struct SafeFileWriter {
    target_path: PathBuf,
    temp_path: PathBuf,
    backup_path: Option<PathBuf>,
}

impl SafeFileWriter {
    /// Create a new safe file writer
    pub fn new<P: AsRef<Path>>(target_path: P) -> Result<Self, io::Error> {
        let target_path = target_path.as_ref().to_path_buf();
        let temp_path = target_path.with_extension("tmp");
        
        let backup_path = if target_path.exists() {
            Some(target_path.with_extension("bak"))
        } else {
            None
        };
        
        Ok(Self {
            target_path,
            temp_path,
            backup_path,
        })
    }
    
    /// Write content to temporary file
    pub fn write(&self, content: &str) -> Result<(), io::Error> {
        write_string(&self.temp_path, content)
    }
    
    /// Write bytes to temporary file
    pub fn write_bytes(&self, content: &[u8]) -> Result<(), io::Error> {
        write_bytes(&self.temp_path, content)
    }
    
    /// Commit the changes (move temp file to target)
    pub fn commit(self) -> Result<(), io::Error> {
        // Create backup if original file exists
        if let Some(backup_path) = &self.backup_path {
            copy_file(&self.target_path, backup_path)?;
        }
        
        // Move temp file to target
        move_path(&self.temp_path, &self.target_path)?;
        
        Ok(())
    }
    
    /// Rollback changes (remove temp file)
    pub fn rollback(self) -> Result<(), io::Error> {
        if self.temp_path.exists() {
            fs::remove_file(&self.temp_path)?;
        }
        Ok(())
    }
    
    /// Get the temporary file path
    pub fn temp_path(&self) -> &Path {
        &self.temp_path
    }
    
    /// Get the target file path
    pub fn target_path(&self) -> &Path {
        &self.target_path
    }
}

impl Drop for SafeFileWriter {
    fn drop(&mut self) {
        // Clean up temp file if it still exists
        if self.temp_path.exists() {
            let _ = fs::remove_file(&self.temp_path);
        }
    }
}

/// File watcher for monitoring file changes
#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub mtime: u64,
    pub exists: bool,
}

impl FileInfo {
    /// Create file info from path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        
        if path.exists() {
            let size = file_size(&path).unwrap_or(0);
            let mtime = file_mtime(&path).unwrap_or(0);
            
            Self {
                path,
                size,
                mtime,
                exists: true,
            }
        } else {
            Self {
                path,
                size: 0,
                mtime: 0,
                exists: false,
            }
        }
    }
    
    /// Check if file has changed
    pub fn has_changed(&self, other: &FileInfo) -> bool {
        self.exists != other.exists
            || self.size != other.size
            || self.mtime != other.mtime
    }
    
    /// Update file info
    pub fn update(&mut self) {
        *self = Self::from_path(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_basic_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Test write and read
        write_string(&file_path, "Hello, World!").unwrap();
        assert!(exists(&file_path));
        assert!(is_file(&file_path));
        
        let content = read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello, World!");
        
        // Test file size
        let size = file_size(&file_path).unwrap();
        assert_eq!(size, 13);
        
        // Test append
        append_string(&file_path, "\nAppended text").unwrap();
        let content = read_to_string(&file_path).unwrap();
        assert!(content.contains("Appended text"));
    }

    #[test]
    fn test_directory_operations() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        
        create_dir_all(&sub_dir).unwrap();
        assert!(exists(&sub_dir));
        assert!(is_dir(&sub_dir));
        
        // Create some files
        let file1 = sub_dir.join("file1.txt");
        let file2 = sub_dir.join("file2.log");
        
        write_string(&file1, "content1").unwrap();
        write_string(&file2, "content2").unwrap();
        
        // List directory
        let entries = list_dir(&sub_dir).unwrap();
        assert_eq!(entries.len(), 2);
        
        // Find files
        let txt_files = find_files(&sub_dir, ".txt", false).unwrap();
        assert_eq!(txt_files.len(), 1);
        
        // Calculate directory size
        let size = dir_size(&sub_dir).unwrap();
        assert!(size > 0);
    }

    #[test]
    fn test_path_utilities() {
        let path = PathBuf::from("/home/user/document.txt");
        
        assert_eq!(get_filename(&path), Some("document.txt".to_string()));
        assert_eq!(get_stem(&path), Some("document".to_string()));
        assert_eq!(get_extension(&path), Some("txt".to_string()));
        
        let joined = join_path("/home/user", "documents/file.txt");
        assert!(joined.to_string_lossy().contains("documents/file.txt"));
    }

    #[test]
    fn test_safe_file_writer() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("safe_test.txt");
        
        // Create initial file
        write_string(&file_path, "original content").unwrap();
        
        // Test safe writing
        {
            let writer = SafeFileWriter::new(&file_path).unwrap();
            writer.write("new content").unwrap();
            writer.commit().unwrap();
        }
        
        let content = read_to_string(&file_path).unwrap();
        assert_eq!(content, "new content");
        
        // Test rollback
        {
            let writer = SafeFileWriter::new(&file_path).unwrap();
            writer.write("temporary content").unwrap();
            writer.rollback().unwrap();
        }
        
        let content = read_to_string(&file_path).unwrap();
        assert_eq!(content, "new content"); // Should be unchanged
    }

    #[test]
    fn test_file_info() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("info_test.txt");
        
        // Test non-existent file
        let info1 = FileInfo::from_path(&file_path);
        assert!(!info1.exists);
        
        // Create file
        write_string(&file_path, "test content").unwrap();
        
        let info2 = FileInfo::from_path(&file_path);
        assert!(info2.exists);
        assert!(info2.size > 0);
        assert!(info2.mtime > 0);
        
        // Test change detection
        assert!(info1.has_changed(&info2));
        
        // Modify file
        std::thread::sleep(std::time::Duration::from_millis(10));
        append_string(&file_path, " more content").unwrap();
        
        let info3 = FileInfo::from_path(&file_path);
        assert!(info2.has_changed(&info3));
    }

    #[test]
    fn test_temp_file_creation() {
        let (file, path) = create_temp_file("test_", ".tmp").unwrap();
        assert!(path.exists());
        
        drop(file);
        fs::remove_file(path).unwrap();
        
        let temp_dir = create_temp_dir("test_dir_").unwrap();
        assert!(temp_dir.exists());
        assert!(temp_dir.is_dir());
        
        fs::remove_dir_all(temp_dir).unwrap();
    }
}