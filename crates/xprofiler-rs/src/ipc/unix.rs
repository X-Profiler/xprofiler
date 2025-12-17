//! Unix domain socket IPC implementation

use super::{IpcClient, IpcError, IpcServer, IpcServerHandle, MessageHandler};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Response socket filename (where server sends responses back to client)
const RESPONSE_SOCKET_FILENAME: &str = "xprofiler-ctl-uds-path.sock";

/// Unix domain socket server
pub struct UnixSocketServer {
    socket_path: PathBuf,
    log_dir: String,
    handle: IpcServerHandle,
}

impl UnixSocketServer {
    pub fn new(log_dir: &str, pid: u32) -> Self {
        let socket_path = PathBuf::from(log_dir).join(format!("xprofiler-uds-path-{}.sock", pid));
        Self {
            socket_path,
            log_dir: log_dir.to_string(),
            handle: IpcServerHandle::new(),
        }
    }

    pub fn socket_path(&self) -> &PathBuf {
        &self.socket_path
    }

    /// Get the response socket path where client listens for responses
    pub fn response_socket_path(&self) -> PathBuf {
        PathBuf::from(&self.log_dir).join(RESPONSE_SOCKET_FILENAME)
    }
}

impl IpcServer for UnixSocketServer {
    fn start(&mut self, handler: MessageHandler) -> Result<(), IpcError> {
        if self.handle.is_running() {
            return Err(IpcError::AlreadyRunning);
        }

        // Remove existing socket file if it exists
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        // Create the Unix listener
        let listener = UnixListener::bind(&self.socket_path)?;
        listener.set_nonblocking(true)?;

        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        self.handle.shutdown_tx = Some(shutdown_tx);

        let socket_path = self.socket_path.clone();
        let response_socket_path = self.response_socket_path();
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_clone = is_running.clone();

        self.handle
            .is_running
            .store(true, Ordering::SeqCst);

        let thread_handle = std::thread::spawn(move || {
            // Create a tokio runtime for this thread
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");

            rt.block_on(async {
                loop {
                    // Check for shutdown signal
                    tokio::select! {
                        _ = shutdown_rx.recv() => {
                            break;
                        }
                        _ = tokio::time::sleep(Duration::from_millis(100)) => {
                            // Try to accept a connection
                            match listener.accept() {
                                Ok((stream, _addr)) => {
                                    let handler = handler.clone();
                                    let resp_path = response_socket_path.clone();
                                    // Handle the connection in a blocking manner
                                    handle_connection(stream, handler, resp_path);
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // No connection available, continue
                                }
                                Err(e) => {
                                    eprintln!("[xprofiler] IPC accept error: {}", e);
                                }
                            }
                        }
                    }
                }
            });

            // Cleanup
            is_running_clone.store(false, Ordering::SeqCst);
            if socket_path.exists() {
                let _ = std::fs::remove_file(&socket_path);
            }
        });

        self.handle.thread_handle = Some(thread_handle);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), IpcError> {
        if !self.handle.is_running() {
            return Err(IpcError::NotRunning);
        }
        self.handle.shutdown();
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.handle.is_running()
    }
}

fn handle_connection(stream: UnixStream, handler: MessageHandler, response_socket_path: PathBuf) {
    // Set the stream to blocking mode (it inherits non-blocking from listener)
    if let Err(e) = stream.set_nonblocking(false) {
        eprintln!("[xprofiler] IPC set blocking error: {}", e);
        return;
    }

    // Set a read timeout
    let _ = stream.set_read_timeout(Some(Duration::from_secs(1)));

    // Read the message (client sends without newline, so use read to buffer)
    let mut buffer = [0u8; 4096];
    let n = match stream.try_clone().unwrap().read(&mut buffer) {
        Ok(0) => {
            // Connection closed
            return;
        }
        Ok(n) => n,
        Err(e) => {
            eprintln!("[xprofiler] IPC read error: {}", e);
            return;
        }
    };

    let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
    if message.is_empty() {
        return;
    }

    // Process the message
    let response = handler(message);

    // Send response to the response socket (not the incoming socket)
    send_response(&response_socket_path, &response);
}

/// Send response to the client's response socket
fn send_response(response_socket_path: &PathBuf, response: &str) {
    // Connect to the client's response socket
    let mut stream = match UnixStream::connect(response_socket_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[xprofiler] Failed to connect to response socket {}: {}",
                     response_socket_path.display(), e);
            return;
        }
    };

    // Set timeouts
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    // Send the response
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("[xprofiler] IPC write error: {}", e);
    }
    let _ = stream.flush();
}

/// Unix domain socket client
pub struct UnixSocketClient {
    socket_path: PathBuf,
}

impl UnixSocketClient {
    pub fn new(socket_path: &str) -> Self {
        Self {
            socket_path: PathBuf::from(socket_path),
        }
    }
}

impl IpcClient for UnixSocketClient {
    fn send(&self, message: &str) -> Result<String, IpcError> {
        // Connect to the socket
        let mut stream = UnixStream::connect(&self.socket_path).map_err(|e| {
            IpcError::ConnectionFailed(format!(
                "Failed to connect to {}: {}",
                self.socket_path.display(),
                e
            ))
        })?;

        // Set timeouts
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;

        // Send the message
        stream
            .write_all(message.as_bytes())
            .map_err(|e| IpcError::SendFailed(e.to_string()))?;
        stream
            .write_all(b"\n")
            .map_err(|e| IpcError::SendFailed(e.to_string()))?;
        stream.flush()?;

        // Read the response
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .map_err(|e| IpcError::ReceiveFailed(e.to_string()))?;

        Ok(response.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unix_socket_server_lifecycle() {
        let log_dir = "/tmp";
        let pid = std::process::id();

        // Create handler
        let handler: MessageHandler = Arc::new(|msg| {
            format!(r#"{{"ok":true,"echo":"{}"}}"#, msg)
        });

        // Start server
        let mut server = UnixSocketServer::new(log_dir, pid);

        server.start(handler).expect("Failed to start server");
        assert!(server.is_running());

        // Give the server time to start
        std::thread::sleep(Duration::from_millis(200));

        // Stop server
        server.stop().expect("Failed to stop server");
        std::thread::sleep(Duration::from_millis(100));
        assert!(!server.is_running());

        // Cleanup
        let socket_path = server.socket_path();
        if socket_path.exists() {
            let _ = std::fs::remove_file(socket_path);
        }
    }

    #[test]
    fn test_response_socket_path() {
        let server = UnixSocketServer::new("/tmp", 12345);
        let response_path = server.response_socket_path();
        assert!(response_path.to_string_lossy().contains("xprofiler-ctl-uds-path.sock"));
    }

    #[test]
    fn test_socket_path_format() {
        let server = UnixSocketServer::new("/var/log", 54321);
        let path = server.socket_path();
        assert!(path.to_string_lossy().contains("xprofiler-uds-path-54321.sock"));
        assert!(path.to_string_lossy().starts_with("/var/log"));
    }
}
