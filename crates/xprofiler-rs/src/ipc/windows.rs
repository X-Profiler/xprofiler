//! Windows named pipe IPC implementation

use super::{IpcClient, IpcError, IpcServer, IpcServerHandle, MessageHandler};
use std::sync::atomic::Ordering;
use std::time::Duration;

/// Windows named pipe server
pub struct NamedPipeServer {
    pipe_name: String,
    handle: IpcServerHandle,
}

impl NamedPipeServer {
    pub fn new(pid: u32) -> Self {
        let pipe_name = format!(r"\\.\pipe\xprofiler-named-pipe-{}", pid);
        Self {
            pipe_name,
            handle: IpcServerHandle::new(),
        }
    }

    pub fn pipe_name(&self) -> &str {
        &self.pipe_name
    }
}

impl IpcServer for NamedPipeServer {
    fn start(&mut self, handler: MessageHandler) -> Result<(), IpcError> {
        if self.handle.is_running() {
            return Err(IpcError::AlreadyRunning);
        }

        // TODO: Implement Windows named pipe server using windows-rs
        // For now, we'll use a placeholder implementation

        #[cfg(windows)]
        {
            use std::sync::Arc;
            use std::sync::atomic::AtomicBool;
            use tokio::sync::mpsc;

            let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
            self.handle.shutdown_tx = Some(shutdown_tx);

            let pipe_name = self.pipe_name.clone();
            let is_running = Arc::new(AtomicBool::new(true));
            let is_running_clone = is_running.clone();

            self.handle.is_running.store(true, Ordering::SeqCst);

            let thread_handle = std::thread::spawn(move || {
                use windows::Win32::Foundation::*;
                use windows::Win32::Storage::FileSystem::{
                    CreateFileW, FlushFileBuffers, ReadFile, WriteFile,
                    FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING,
                };
                use windows::Win32::System::Pipes::*;
                use windows::core::*;

                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create tokio runtime");

                rt.block_on(async {
                    loop {
                        tokio::select! {
                            _ = shutdown_rx.recv() => {
                                break;
                            }
                            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                                // Create named pipe instance
                                let pipe_name_wide: Vec<u16> = pipe_name
                                    .encode_utf16()
                                    .chain(std::iter::once(0))
                                    .collect();

                                unsafe {
                                    let pipe = CreateNamedPipeW(
                                        PCWSTR::from_raw(pipe_name_wide.as_ptr()),
                                        PIPE_ACCESS_DUPLEX,
                                        PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                                        PIPE_UNLIMITED_INSTANCES,
                                        4096,
                                        4096,
                                        0,
                                        None,
                                    );

                                    if pipe == INVALID_HANDLE_VALUE {
                                        continue;
                                    }

                                    // Wait for client connection (non-blocking check)
                                    if ConnectNamedPipe(pipe, None).is_ok() ||
                                       GetLastError() == ERROR_PIPE_CONNECTED {
                                        // Handle the connection
                                        let mut buffer = vec![0u8; 4096];
                                        let mut bytes_read = 0u32;

                                        if ReadFile(
                                            pipe,
                                            Some(&mut buffer),
                                            Some(&mut bytes_read),
                                            None,
                                        ).is_ok() && bytes_read > 0 {
                                            let message = String::from_utf8_lossy(
                                                &buffer[..bytes_read as usize]
                                            ).trim().to_string();

                                            let response = handler(message);
                                            let response_bytes = response.as_bytes();
                                            let mut bytes_written = 0u32;

                                            let _ = WriteFile(
                                                pipe,
                                                Some(response_bytes),
                                                Some(&mut bytes_written),
                                                None,
                                            );

                                            let _ = FlushFileBuffers(pipe);
                                        }

                                        let _ = DisconnectNamedPipe(pipe);
                                    }

                                    let _ = CloseHandle(pipe);
                                }
                            }
                        }
                    }
                });

                is_running_clone.store(false, Ordering::SeqCst);
            });

            self.handle.thread_handle = Some(thread_handle);
        }

        #[cfg(not(windows))]
        {
            let _ = handler;
            // On non-Windows, just mark as running for testing
            self.handle.is_running.store(true, Ordering::SeqCst);
        }

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

/// Windows named pipe client
pub struct NamedPipeClient {
    pipe_name: String,
}

impl NamedPipeClient {
    pub fn new(pipe_name: &str) -> Self {
        Self {
            pipe_name: pipe_name.to_string(),
        }
    }
}

impl IpcClient for NamedPipeClient {
    fn send(&self, message: &str) -> Result<String, IpcError> {
        #[cfg(windows)]
        {
            use windows::Win32::Foundation::*;
            use windows::Win32::Storage::FileSystem::{
                CreateFileW, ReadFile, WriteFile,
                FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING,
            };
            use windows::Win32::System::Pipes::*;
            use windows::core::*;

            let pipe_name_wide: Vec<u16> = self.pipe_name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            unsafe {
                // Try to open the named pipe
                let pipe = CreateFileW(
                    PCWSTR::from_raw(pipe_name_wide.as_ptr()),
                    GENERIC_READ.0 | GENERIC_WRITE.0,
                    FILE_SHARE_NONE,
                    None,
                    OPEN_EXISTING,
                    FILE_ATTRIBUTE_NORMAL,
                    None,
                ).map_err(|e| IpcError::ConnectionFailed(format!("Failed to open pipe: {}", e)))?;

                // Set the pipe to message mode
                let mut mode = PIPE_READMODE_MESSAGE;
                SetNamedPipeHandleState(pipe, Some(&mut mode), None, None)
                    .map_err(|e| IpcError::ConnectionFailed(format!("Failed to set pipe mode: {}", e)))?;

                // Write message
                let message_bytes = message.as_bytes();
                let mut bytes_written = 0u32;
                WriteFile(pipe, Some(message_bytes), Some(&mut bytes_written), None)
                    .map_err(|e| IpcError::SendFailed(format!("Failed to write: {}", e)))?;

                // Read response
                let mut buffer = vec![0u8; 4096];
                let mut bytes_read = 0u32;
                ReadFile(pipe, Some(&mut buffer), Some(&mut bytes_read), None)
                    .map_err(|e| IpcError::ReceiveFailed(format!("Failed to read: {}", e)))?;

                let _ = CloseHandle(pipe);

                let response = String::from_utf8_lossy(&buffer[..bytes_read as usize])
                    .trim()
                    .to_string();
                Ok(response)
            }
        }

        #[cfg(not(windows))]
        {
            let _ = message;
            Err(IpcError::ConnectionFailed(
                "Named pipes only supported on Windows".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_name_format() {
        let server = NamedPipeServer::new(12345);
        assert_eq!(server.pipe_name(), r"\\.\pipe\xprofiler-named-pipe-12345");
    }
}
