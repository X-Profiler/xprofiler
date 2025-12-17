//! IPC (Inter-Process Communication) module for xprofiler-rs
//!
//! This module provides cross-platform IPC implementations:
//! - Unix: Unix domain sockets
//! - Windows: Named pipes

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

use std::sync::Arc;
use tokio::sync::mpsc;

/// Message handler callback type
pub type MessageHandler = Arc<dyn Fn(String) -> String + Send + Sync>;

/// IPC server trait
pub trait IpcServer: Send + Sync {
    /// Start the IPC server
    fn start(&mut self, handler: MessageHandler) -> Result<(), IpcError>;

    /// Stop the IPC server
    fn stop(&mut self) -> Result<(), IpcError>;

    /// Check if the server is running
    fn is_running(&self) -> bool;
}

/// IPC client trait
pub trait IpcClient: Send + Sync {
    /// Send a message and receive a response
    fn send(&self, message: &str) -> Result<String, IpcError>;
}

/// IPC error types
#[derive(Debug, thiserror::Error)]
pub enum IpcError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Server already running")]
    AlreadyRunning,

    #[error("Server not running")]
    NotRunning,

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Send failed: {0}")]
    SendFailed(String),

    #[error("Receive failed: {0}")]
    ReceiveFailed(String),

    #[error("Timeout")]
    Timeout,

    #[error("Server shutdown")]
    Shutdown,
}

/// Create an IPC server for the current platform
#[cfg(unix)]
pub fn create_server(log_dir: &str, pid: u32) -> Box<dyn IpcServer> {
    Box::new(unix::UnixSocketServer::new(log_dir, pid))
}

#[cfg(windows)]
pub fn create_server(_log_dir: &str, pid: u32) -> Box<dyn IpcServer> {
    Box::new(windows::NamedPipeServer::new(pid))
}

/// Create an IPC client for the current platform
#[cfg(unix)]
pub fn create_client(socket_path: &str) -> Box<dyn IpcClient> {
    Box::new(unix::UnixSocketClient::new(socket_path))
}

#[cfg(windows)]
pub fn create_client(pipe_name: &str) -> Box<dyn IpcClient> {
    Box::new(windows::NamedPipeClient::new(pipe_name))
}

/// IPC server handle for managing the background thread
pub struct IpcServerHandle {
    shutdown_tx: Option<mpsc::Sender<()>>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    is_running: std::sync::atomic::AtomicBool,
}

impl IpcServerHandle {
    pub fn new() -> Self {
        Self {
            shutdown_tx: None,
            thread_handle: None,
            is_running: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.blocking_send(());
        }
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
        self.is_running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

impl Default for IpcServerHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IpcServerHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}
