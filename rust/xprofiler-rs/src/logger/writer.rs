//! Log writers for different output destinations

use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
// use std::time::{SystemTime, UNIX_EPOCH}; // Commented out unused imports

/// Trait for log writers
pub trait LogWriter {
    /// Write a formatted log message
    fn write(&self, message: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Flush any buffered content
    fn flush(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Console writer that outputs to stdout/stderr
pub struct ConsoleWriter {
    use_stderr: bool,
}

impl ConsoleWriter {
    /// Create a new console writer that outputs to stdout
    pub fn new() -> Self {
        Self { use_stderr: false }
    }
    
    /// Create a new console writer that outputs to stderr
    pub fn stderr() -> Self {
        Self { use_stderr: true }
    }
}

impl LogWriter for ConsoleWriter {
    fn write(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.use_stderr {
            eprint!("{}", message);
        } else {
            print!("{}", message);
        }
        Ok(())
    }
    
    fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.use_stderr {
            io::stderr().flush()?;
        } else {
            io::stdout().flush()?;
        }
        Ok(())
    }
}

/// File writer with rotation support
pub struct FileWriter {
    file_path: PathBuf,
    max_size: u64,
    max_files: u32,
    writer: Arc<Mutex<BufWriter<File>>>,
    current_size: Arc<Mutex<u64>>,
}

impl FileWriter {
    /// Create a new file writer
    pub fn new<P: AsRef<Path>>(
        file_path: P,
        max_size: u64,
        max_files: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref().to_path_buf();
        
        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Open or create the log file
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;
        
        let current_size = file.metadata()?.len();
        let writer = Arc::new(Mutex::new(BufWriter::new(file)));
        
        Ok(Self {
            file_path,
            max_size,
            max_files,
            writer,
            current_size: Arc::new(Mutex::new(current_size)),
        })
    }
    
    /// Rotate log files if necessary
    fn rotate_if_needed(&self, message_size: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_size = self.current_size.lock().map_err(|e| {
            format!("Failed to acquire size lock: {}", e)
        })?;
        
        if *current_size + message_size > self.max_size {
            // Rotate files
            self.rotate_files()?;
            
            // Create new file
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.file_path)?;
            
            let mut writer = self.writer.lock().map_err(|e| {
                format!("Failed to acquire writer lock: {}", e)
            })?;
            
            *writer = BufWriter::new(file);
            *current_size = 0;
        }
        
        Ok(())
    }
    
    /// Rotate log files
    fn rotate_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Remove the oldest file if it exists
        let oldest_file = self.get_rotated_filename(self.max_files - 1);
        if oldest_file.exists() {
            std::fs::remove_file(&oldest_file)?;
        }
        
        // Rotate existing files
        for i in (1..self.max_files - 1).rev() {
            let from = self.get_rotated_filename(i);
            let to = self.get_rotated_filename(i + 1);
            
            if from.exists() {
                std::fs::rename(&from, &to)?;
            }
        }
        
        // Move current file to .1
        if self.file_path.exists() {
            let rotated = self.get_rotated_filename(1);
            std::fs::rename(&self.file_path, &rotated)?;
        }
        
        Ok(())
    }
    
    /// Get the filename for a rotated log file
    fn get_rotated_filename(&self, index: u32) -> PathBuf {
        let mut rotated = self.file_path.clone();
        let extension = format!(".{}", index);
        
        if let Some(current_extension) = rotated.extension() {
            let new_extension = format!("{}{}", current_extension.to_string_lossy(), extension);
            rotated.set_extension(new_extension);
        } else {
            let filename = rotated.file_name().unwrap().to_string_lossy();
            rotated.set_file_name(format!("{}{}", filename, extension));
        }
        
        rotated
    }
}

impl LogWriter for FileWriter {
    fn write(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message_size = message.len() as u64;
        
        // Check if rotation is needed
        self.rotate_if_needed(message_size)?;
        
        // Write the message
        {
            let mut writer = self.writer.lock().map_err(|e| {
                format!("Failed to acquire writer lock: {}", e)
            })?;
            
            writer.write_all(message.as_bytes())?;
        }
        
        // Update current size
        {
            let mut current_size = self.current_size.lock().map_err(|e| {
                format!("Failed to acquire size lock: {}", e)
            })?;
            
            *current_size += message_size;
        }
        
        Ok(())
    }
    
    fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = self.writer.lock().map_err(|e| {
            format!("Failed to acquire writer lock: {}", e)
        })?;
        
        writer.flush()?;
        Ok(())
    }
}

/// Multi-writer that can write to multiple destinations
pub struct MultiWriter {
    writers: Vec<Box<dyn LogWriter + Send + Sync>>,
}

impl MultiWriter {
    /// Create a new multi-writer
    pub fn new() -> Self {
        Self {
            writers: Vec::new(),
        }
    }
    
    /// Add a writer to the multi-writer
    pub fn add_writer(mut self, writer: Box<dyn LogWriter + Send + Sync>) -> Self {
        self.writers.push(writer);
        self
    }
    
    /// Get the number of writers
    pub fn writer_count(&self) -> usize {
        self.writers.len()
    }
}

impl LogWriter for MultiWriter {
    fn write(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut errors = Vec::new();
        
        for writer in &self.writers {
            if let Err(e) = writer.write(message) {
                errors.push(e.to_string());
            }
        }
        
        if !errors.is_empty() {
            return Err(format!("Multiple write errors: {}", errors.join(", ")).into());
        }
        
        Ok(())
    }
    
    fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut errors = Vec::new();
        
        for writer in &self.writers {
            if let Err(e) = writer.flush() {
                errors.push(e.to_string());
            }
        }
        
        if !errors.is_empty() {
            return Err(format!("Multiple flush errors: {}", errors.join(", ")).into());
        }
        
        Ok(())
    }
}

/// Buffered writer for async logging
pub struct BufferedWriter {
    inner: Box<dyn LogWriter + Send + Sync>,
    buffer: Arc<Mutex<VecDeque<String>>>,
    buffer_size: usize,
    auto_flush: bool,
}

impl BufferedWriter {
    /// Create a new buffered writer
    pub fn new(
        inner: Box<dyn LogWriter + Send + Sync>,
        buffer_size: usize,
    ) -> Self {
        Self {
            inner,
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(buffer_size))),
            buffer_size,
            auto_flush: true,
        }
    }
    
    /// Set auto-flush behavior
    pub fn with_auto_flush(mut self, auto_flush: bool) -> Self {
        self.auto_flush = auto_flush;
        self
    }
    
    /// Flush the buffer to the inner writer
    fn flush_buffer(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = self.buffer.lock().map_err(|e| {
            format!("Failed to acquire buffer lock: {}", e)
        })?;
        
        while let Some(message) = buffer.pop_front() {
            self.inner.write(&message)?;
        }
        
        self.inner.flush()?;
        Ok(())
    }
    
    /// Get current buffer size
    pub fn buffer_len(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let buffer = self.buffer.lock().map_err(|e| {
            format!("Failed to acquire buffer lock: {}", e)
        })?;
        
        Ok(buffer.len())
    }
}

impl LogWriter for BufferedWriter {
    fn write(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = self.buffer.lock().map_err(|e| {
            format!("Failed to acquire buffer lock: {}", e)
        })?;
        
        buffer.push_back(message.to_string());
        
        // Check if buffer is full or auto-flush is enabled
        let should_flush = buffer.len() >= self.buffer_size || self.auto_flush;
        
        drop(buffer); // Release lock before flushing
        
        if should_flush {
            self.flush_buffer()?;
        }
        
        Ok(())
    }
    
    fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.flush_buffer()
    }
}

/// Null writer that discards all output (useful for testing)
pub struct NullWriter;

impl NullWriter {
    pub fn new() -> Self {
        Self
    }
}

impl LogWriter for NullWriter {
    fn write(&self, _message: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_console_writer() {
        let writer = ConsoleWriter::new();
        assert!(writer.write("test message\n").is_ok());
        assert!(writer.flush().is_ok());
    }

    #[test]
    fn test_console_writer_stderr() {
        let writer = ConsoleWriter::stderr();
        assert!(writer.write("test error message\n").is_ok());
        assert!(writer.flush().is_ok());
    }

    #[test]
    fn test_file_writer() {
        let temp_dir = TempDir::new().unwrap();
        let log_file = temp_dir.path().join("test.log");
        
        let writer = FileWriter::new(&log_file, 1024, 3).unwrap();
        
        writer.write("test message 1\n").unwrap();
        writer.write("test message 2\n").unwrap();
        writer.flush().unwrap();
        
        let content = fs::read_to_string(&log_file).unwrap();
        assert!(content.contains("test message 1"));
        assert!(content.contains("test message 2"));
    }

    #[test]
    fn test_file_writer_rotation() {
        let temp_dir = TempDir::new().unwrap();
        let log_file = temp_dir.path().join("test.log");
        
        // Create a writer with very small max size to trigger rotation
        let writer = FileWriter::new(&log_file, 20, 3).unwrap();
        
        writer.write("short message\n").unwrap();
        writer.write("this is a longer message that should trigger rotation\n").unwrap();
        writer.flush().unwrap();
        
        // Check that rotation occurred
        let rotated_file = temp_dir.path().join("test.log.1");
        assert!(rotated_file.exists());
    }

    #[test]
    fn test_multi_writer() {
        let temp_dir = TempDir::new().unwrap();
        let log_file = temp_dir.path().join("test.log");
        
        let console_writer = Box::new(ConsoleWriter::new());
        let file_writer = Box::new(FileWriter::new(&log_file, 1024, 3).unwrap());
        
        let multi_writer = MultiWriter::new()
            .add_writer(console_writer)
            .add_writer(file_writer);
        
        assert_eq!(multi_writer.writer_count(), 2);
        
        multi_writer.write("test message\n").unwrap();
        multi_writer.flush().unwrap();
        
        let content = fs::read_to_string(&log_file).unwrap();
        assert!(content.contains("test message"));
    }

    #[test]
    fn test_buffered_writer() {
        let temp_dir = TempDir::new().unwrap();
        let log_file = temp_dir.path().join("test.log");
        
        let file_writer = Box::new(FileWriter::new(&log_file, 1024, 3).unwrap());
        let buffered_writer = BufferedWriter::new(file_writer, 3);
        
        buffered_writer.write("message 1\n").unwrap();
        buffered_writer.write("message 2\n").unwrap();
        
        // Buffer should contain messages
        assert_eq!(buffered_writer.buffer_len().unwrap(), 2);
        
        buffered_writer.flush().unwrap();
        
        // Buffer should be empty after flush
        assert_eq!(buffered_writer.buffer_len().unwrap(), 0);
        
        let content = fs::read_to_string(&log_file).unwrap();
        assert!(content.contains("message 1"));
        assert!(content.contains("message 2"));
    }

    #[test]
    fn test_null_writer() {
        let writer = NullWriter::new();
        assert!(writer.write("test message\n").is_ok());
        assert!(writer.flush().is_ok());
    }
}