//! Command parser and dispatcher

use super::{CommandRequest, CommandResponse};
use crate::config;
use crate::env::EnvironmentRegistry;

/// Parse and dispatch a command
pub fn handle_command(input: &str) -> String {
    // Parse the command
    let request: CommandRequest = match serde_json::from_str(input) {
        Ok(req) => req,
        Err(e) => {
            let response = CommandResponse::error("unknown", &format!("Invalid JSON: {}", e));
            return serde_json::to_string(&response).unwrap_or_default();
        }
    };

    // Dispatch the command
    let response = dispatch_command(&request);

    // Serialize the response
    serde_json::to_string(&response).unwrap_or_else(|_| {
        r#"{"ok":false,"traceid":"unknown","message":"Failed to serialize response"}"#.to_string()
    })
}

/// Dispatch a command to the appropriate handler
fn dispatch_command(request: &CommandRequest) -> CommandResponse {
    match request.cmd.as_str() {
        "check_version" => handle_check_version(request),
        "list_environments" => handle_list_environments(request),
        "get_config" => handle_get_config(request),
        "set_config" => handle_set_config(request),
        // Profiling commands - TODO: implement in Phase 5
        "start_cpu_profiling" => handle_not_implemented(request, "CPU profiling not yet implemented in Rust"),
        "stop_cpu_profiling" => handle_not_implemented(request, "CPU profiling not yet implemented in Rust"),
        "heapdump" => handle_not_implemented(request, "Heap dump not yet implemented in Rust"),
        "start_heap_profiling" => handle_not_implemented(request, "Heap profiling not yet implemented in Rust"),
        "stop_heap_profiling" => handle_not_implemented(request, "Heap profiling not yet implemented in Rust"),
        "start_gc_profiling" => handle_not_implemented(request, "GC profiling not yet implemented in Rust"),
        "stop_gc_profiling" => handle_not_implemented(request, "GC profiling not yet implemented in Rust"),
        "diag_report" => handle_not_implemented(request, "Diagnostic report not yet implemented in Rust"),
        _ => CommandResponse::error(&request.traceid, &format!("Unknown command: {}", request.cmd)),
    }
}

/// Handle check_version command
fn handle_check_version(request: &CommandRequest) -> CommandResponse {
    CommandResponse::success(
        &request.traceid,
        Some(serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "node_version": get_node_version(),
        })),
    )
}

/// Handle list_environments command
fn handle_list_environments(request: &CommandRequest) -> CommandResponse {
    let registry = EnvironmentRegistry::global();
    let mut environments = Vec::new();

    registry.for_each(|env| {
        environments.push(serde_json::json!({
            "thread_id": env.thread_id,
            "is_main_thread": env.is_main_thread,
            "node_version": env.node_version,
        }));
    });

    CommandResponse::success(
        &request.traceid,
        Some(serde_json::json!({
            "count": environments.len(),
            "environments": environments,
        })),
    )
}

/// Handle get_config command
fn handle_get_config(request: &CommandRequest) -> CommandResponse {
    let cfg = config::get_config();
    match serde_json::to_value(cfg) {
        Ok(value) => CommandResponse::success(&request.traceid, Some(value)),
        Err(e) => CommandResponse::error(&request.traceid, &format!("Failed to serialize config: {}", e)),
    }
}

/// Handle set_config command
fn handle_set_config(request: &CommandRequest) -> CommandResponse {
    // TODO: Implement config update from options
    // For now, just return success
    CommandResponse::success(&request.traceid, Some(serde_json::json!({
        "message": "Config update not yet implemented"
    })))
}

/// Handle not implemented commands
fn handle_not_implemented(request: &CommandRequest, message: &str) -> CommandResponse {
    CommandResponse::error(&request.traceid, message)
}

/// Get the Node.js version from the main thread environment
fn get_node_version() -> String {
    EnvironmentRegistry::global()
        .get_main_thread()
        .map(|env| env.node_version.clone())
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_invalid_json() {
        let response = handle_command("not valid json");
        assert!(response.contains("Invalid JSON"));
        assert!(response.contains(r#""ok":false"#));
    }

    #[test]
    fn test_handle_check_version() {
        let input = r#"{"traceid":"test-123","cmd":"check_version"}"#;
        let response = handle_command(input);
        assert!(response.contains(r#""ok":true"#));
        assert!(response.contains("version"));
    }

    #[test]
    fn test_handle_unknown_command() {
        let input = r#"{"traceid":"test-123","cmd":"unknown_command"}"#;
        let response = handle_command(input);
        assert!(response.contains(r#""ok":false"#));
        assert!(response.contains("Unknown command"));
    }

    #[test]
    fn test_handle_get_config() {
        let input = r#"{"traceid":"test-123","cmd":"get_config"}"#;
        let response = handle_command(input);
        assert!(response.contains(r#""ok":true"#));
        assert!(response.contains("log_dir"));
    }
}
