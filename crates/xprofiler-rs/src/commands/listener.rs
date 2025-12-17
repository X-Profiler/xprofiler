//! Commands listener thread management

use super::parser::handle_command;
use crate::config;
use crate::ipc::{self, IpcServer, MessageHandler};
use crate::utils;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::Arc;

/// Global commands listener
static COMMANDS_LISTENER: Lazy<Mutex<Option<CommandsListener>>> = Lazy::new(|| Mutex::new(None));

/// Commands listener state
struct CommandsListener {
    server: Box<dyn IpcServer>,
}

/// Start the commands listener thread
pub fn start_commands_listener() -> Result<(), String> {
    let mut listener = COMMANDS_LISTENER.lock();

    if listener.is_some() {
        return Err("Commands listener already started".to_string());
    }

    let cfg = config::get_config();
    let pid = utils::get_pid();

    // Create the IPC server
    let mut server = ipc::create_server(&cfg.log_dir, pid);

    // Create the message handler
    let handler: MessageHandler = Arc::new(|message| {
        handle_command(&message)
    });

    // Start the server
    server.start(handler).map_err(|e| format!("Failed to start IPC server: {}", e))?;

    *listener = Some(CommandsListener { server });

    Ok(())
}

/// Stop the commands listener thread
pub fn stop_commands_listener() -> Result<(), String> {
    let mut listener = COMMANDS_LISTENER.lock();

    if let Some(mut l) = listener.take() {
        l.server.stop().map_err(|e| format!("Failed to stop IPC server: {}", e))?;
    }

    Ok(())
}

/// Check if the commands listener is running
pub fn is_commands_listener_running() -> bool {
    COMMANDS_LISTENER
        .lock()
        .as_ref()
        .map_or(false, |l| l.server.is_running())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_stop_listener() {
        // Set up a test config with a temp directory
        let temp_dir = std::env::temp_dir().join("xprofiler-test-listener");
        std::fs::create_dir_all(&temp_dir).ok();

        config::update_config(|c| {
            c.log_dir = temp_dir.to_string_lossy().to_string();
        }).unwrap();

        // Start the listener
        let result = start_commands_listener();
        assert!(result.is_ok(), "Failed to start: {:?}", result);
        assert!(is_commands_listener_running());

        // Try to start again (should fail)
        let result = start_commands_listener();
        assert!(result.is_err());

        // Stop the listener
        let result = stop_commands_listener();
        assert!(result.is_ok());
        assert!(!is_commands_listener_running());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
