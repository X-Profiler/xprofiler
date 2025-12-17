//! Unix domain socket IPC implementation

use super::{IpcClient, IpcError, IpcServer, IpcServerHandle, MessageHandler};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Unix domain socket server
pub struct UnixSocketServer {
    socket_path: PathBuf,
    handle: IpcServerHandle,
}

impl UnixSocketServer {
    pub fn new(log_dir: &str, pid: u32) -> Self {
        let socket_path = PathBuf::from(log_dir).join(format!("xprofiler-uds-path-{}.sock", pid));
        Self {
            socket_path,
            handle: IpcServerHandle::new(),
        }
    }

    pub fn socket_path(&self) -> &PathBuf {
        &self.socket_path
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
                                    // Handle the connection in a blocking manner
                                    // (Unix sockets are typically fast enough)
                                    handle_connection(stream, handler);
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

fn handle_connection(mut stream: UnixStream, handler: MessageHandler) {
    // Set a read timeout
    let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));

    let mut reader = BufReader::new(stream.try_clone().unwrap_or_else(|_| {
        // If clone fails, we can't read and write separately
        return stream.try_clone().unwrap();
    }));

    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => {
            // Connection closed
            return;
        }
        Ok(_) => {
            // Process the message
            let response = handler(line.trim().to_string());

            // Send response
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("[xprofiler] IPC write error: {}", e);
            }
            if let Err(e) = stream.write_all(b"\n") {
                eprintln!("[xprofiler] IPC write newline error: {}", e);
            }
            let _ = stream.flush();
        }
        Err(e) => {
            eprintln!("[xprofiler] IPC read error: {}", e);
        }
    }
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
    use std::sync::atomic::AtomicU32;

    static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn get_test_socket_path() -> String {
        let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!(
            "/tmp/xprofiler-test-{}-{}.sock",
            std::process::id(),
            id
        )
    }

    #[test]
    fn test_unix_socket_server_client() {
        let socket_path = get_test_socket_path();
        let log_dir = "/tmp";
        let pid = std::process::id();

        // Create handler
        let handler: MessageHandler = Arc::new(|msg| {
            format!(r#"{{"ok":true,"echo":"{}"}}"#, msg)
        });

        // Start server
        let mut server = UnixSocketServer::new(log_dir, pid);
        // Override the socket path for testing
        server.socket_path = PathBuf::from(&socket_path);

        server.start(handler).expect("Failed to start server");
        assert!(server.is_running());

        // Give the server time to start
        std::thread::sleep(Duration::from_millis(200));

        // Create client and send message
        let client = UnixSocketClient::new(&socket_path);
        let response = client.send(r#"{"cmd":"test"}"#).expect("Failed to send");

        assert!(response.contains("ok"));
        assert!(response.contains("true"));

        // Stop server
        server.stop().expect("Failed to stop server");
        assert!(!server.is_running());

        // Cleanup
        if PathBuf::from(&socket_path).exists() {
            let _ = std::fs::remove_file(&socket_path);
        }
    }
}
