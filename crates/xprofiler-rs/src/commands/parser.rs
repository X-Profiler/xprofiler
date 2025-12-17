//! Command parser and dispatcher

use super::{CommandRequest, CommandResponse};
use crate::config;
use crate::env::EnvironmentRegistry;
use crate::profilers::{
    self, check_dependent_action, create_filepath, get_profile_state, is_action_running,
    set_action_running, ProfileAction,
};
use crate::profilers::gc;

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
        // CPU profiling - requires V8 profiler (not yet implemented)
        "start_cpu_profiling" => handle_start_cpu_profiling(request),
        "stop_cpu_profiling" => handle_stop_cpu_profiling(request),
        // Heap dump - requires V8 profiler (not yet implemented)
        "heapdump" => handle_heapdump(request),
        // Heap profiling - requires V8 profiler (not yet implemented)
        "start_heap_profiling" => handle_start_heap_profiling(request),
        "stop_heap_profiling" => handle_stop_heap_profiling(request),
        // GC profiling - fully implemented in Rust
        "start_gc_profiling" => handle_start_gc_profiling(request),
        "stop_gc_profiling" => handle_stop_gc_profiling(request),
        // Diagnostic report - not yet implemented
        "diag_report" => handle_not_implemented(request, "Diagnostic report not yet implemented in Rust"),
        // Coredump - not yet implemented
        "generate_coredump" => handle_generate_coredump(request),
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
            "uptime": crate::utils::get_uptime(),
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
    // Parse options and update config
    if let Some(_options) = &request.options {
        // For now, return success with the current config
        // TODO: Implement actual config update
    }

    let cfg = config::get_config();
    match serde_json::to_value(cfg) {
        Ok(value) => CommandResponse::success(&request.traceid, Some(value)),
        Err(e) => CommandResponse::error(&request.traceid, &format!("Failed to serialize config: {}", e)),
    }
}

/// Get thread_id from request, defaulting to main thread (0)
fn get_thread_id(request: &CommandRequest) -> i64 {
    request.thread_id.unwrap_or(0)
}

/// Handle start_cpu_profiling command
fn handle_start_cpu_profiling(request: &CommandRequest) -> CommandResponse {
    let thread_id = get_thread_id(request);

    // Check if already running
    if is_action_running(thread_id, ProfileAction::StartCpuProfiling) {
        return CommandResponse::error(&request.traceid, "start_cpu_profiling is running.");
    }

    // Create filepath
    let filepath = create_filepath("cpuprofile", "cpuprofile");

    // Store filepath in profile state
    {
        let mut state = get_profile_state(thread_id);
        state.cpuprofile_filepath = Some(filepath.clone());
    }

    // Set action as running
    set_action_running(thread_id, ProfileAction::StartCpuProfiling);

    // Note: Actual V8 CPU profiling would need to be triggered via JavaScript
    // For now, we just set up the state

    CommandResponse::success(
        &request.traceid,
        Some(serde_json::json!({
            "filepath": filepath,
        })),
    )
}

/// Handle stop_cpu_profiling command
fn handle_stop_cpu_profiling(request: &CommandRequest) -> CommandResponse {
    let thread_id = get_thread_id(request);

    // Check dependent action
    if let Err(msg) = check_dependent_action(thread_id, ProfileAction::StopCpuProfiling) {
        return CommandResponse::error(&request.traceid, &msg);
    }

    // Get filepath
    let filepath = {
        let state = get_profile_state(thread_id);
        state.cpuprofile_filepath.clone()
    };

    // Clear actions
    profilers::clear_action(thread_id, ProfileAction::StartCpuProfiling);
    profilers::clear_action(thread_id, ProfileAction::StopCpuProfiling);

    match filepath {
        Some(fp) => CommandResponse::success(
            &request.traceid,
            Some(serde_json::json!({
                "filepath": fp,
            })),
        ),
        None => CommandResponse::error(&request.traceid, "No CPU profile filepath found"),
    }
}

/// Handle heapdump command
fn handle_heapdump(request: &CommandRequest) -> CommandResponse {
    let thread_id = get_thread_id(request);

    // Check if already running
    if is_action_running(thread_id, ProfileAction::Heapdump) {
        return CommandResponse::error(&request.traceid, "heapdump is running.");
    }

    // Create filepath
    let filepath = create_filepath("heapdump", "heapsnapshot");

    // Store filepath
    {
        let mut state = get_profile_state(thread_id);
        state.heapsnapshot_filepath = Some(filepath.clone());
    }

    // Set action as running
    set_action_running(thread_id, ProfileAction::Heapdump);

    // Note: Actual V8 heap snapshot would need to be triggered via JavaScript

    CommandResponse::success(
        &request.traceid,
        Some(serde_json::json!({
            "filepath": filepath,
        })),
    )
}

/// Handle start_heap_profiling command
fn handle_start_heap_profiling(request: &CommandRequest) -> CommandResponse {
    let thread_id = get_thread_id(request);

    // Check if already running
    if is_action_running(thread_id, ProfileAction::StartHeapProfiling) {
        return CommandResponse::error(&request.traceid, "start_sampling_heap_profiling is running.");
    }

    // Create filepath
    let filepath = create_filepath("heapprofile", "heapprofile");

    // Store filepath
    {
        let mut state = get_profile_state(thread_id);
        state.heapprofile_filepath = Some(filepath.clone());
    }

    // Set action as running
    set_action_running(thread_id, ProfileAction::StartHeapProfiling);

    CommandResponse::success(
        &request.traceid,
        Some(serde_json::json!({
            "filepath": filepath,
        })),
    )
}

/// Handle stop_heap_profiling command
fn handle_stop_heap_profiling(request: &CommandRequest) -> CommandResponse {
    let thread_id = get_thread_id(request);

    // Check dependent action
    if let Err(msg) = check_dependent_action(thread_id, ProfileAction::StopHeapProfiling) {
        return CommandResponse::error(&request.traceid, &msg);
    }

    // Get filepath
    let filepath = {
        let state = get_profile_state(thread_id);
        state.heapprofile_filepath.clone()
    };

    // Clear actions
    profilers::clear_action(thread_id, ProfileAction::StartHeapProfiling);
    profilers::clear_action(thread_id, ProfileAction::StopHeapProfiling);

    match filepath {
        Some(fp) => CommandResponse::success(
            &request.traceid,
            Some(serde_json::json!({
                "filepath": fp,
            })),
        ),
        None => CommandResponse::error(&request.traceid, "No heap profile filepath found"),
    }
}

/// Handle start_gc_profiling command
fn handle_start_gc_profiling(request: &CommandRequest) -> CommandResponse {
    let thread_id = get_thread_id(request);

    // Check if already running
    if is_action_running(thread_id, ProfileAction::StartGcProfiling) {
        return CommandResponse::error(&request.traceid, "start_gc_profiling is running.");
    }

    // Create filepath
    let filepath = create_filepath("gcprofile", "gcprofile");

    // Store filepath in profile state
    {
        let mut state = get_profile_state(thread_id);
        state.gcprofile_filepath = Some(filepath.clone());
    }

    // Start GC profiling
    if let Err(e) = gc::start_gc_profiling(thread_id, filepath.clone()) {
        return CommandResponse::error(&request.traceid, &e);
    }

    CommandResponse::success(
        &request.traceid,
        Some(serde_json::json!({
            "filepath": filepath,
        })),
    )
}

/// Handle stop_gc_profiling command
fn handle_stop_gc_profiling(request: &CommandRequest) -> CommandResponse {
    let thread_id = get_thread_id(request);

    // Check dependent action
    if let Err(msg) = check_dependent_action(thread_id, ProfileAction::StopGcProfiling) {
        return CommandResponse::error(&request.traceid, &msg);
    }

    // Stop GC profiling
    match gc::stop_gc_profiling(thread_id) {
        Ok(filepath) => CommandResponse::success(
            &request.traceid,
            Some(serde_json::json!({
                "filepath": filepath,
            })),
        ),
        Err(e) => CommandResponse::error(&request.traceid, &e),
    }
}

/// Handle generate_coredump command
fn handle_generate_coredump(request: &CommandRequest) -> CommandResponse {
    #[cfg(target_os = "linux")]
    {
        let thread_id = get_thread_id(request);
        let filepath = create_filepath("coredump", "core");

        // Store filepath
        {
            let mut state = get_profile_state(thread_id);
            state.coredump_filepath = Some(filepath.clone());
        }

        // Attempt to generate coredump
        let result = crate::coredump::write_coredump(&filepath);

        if result.success {
            CommandResponse::success(
                &request.traceid,
                Some(serde_json::json!({
                    "filepath": result.filepath.unwrap_or(filepath),
                    "message": result.message,
                })),
            )
        } else {
            // Return success with guidance (coredump info file was created)
            CommandResponse::success(
                &request.traceid,
                Some(serde_json::json!({
                    "filepath": result.filepath.unwrap_or(filepath),
                    "message": result.message,
                    "guidance": crate::coredump::get_coredump_guidance(),
                })),
            )
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        CommandResponse::error(
            &request.traceid,
            "generate_coredump only support linux now.",
        )
    }
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

    #[test]
    fn test_handle_stop_gc_profiling_without_start() {
        let input = r#"{"traceid":"test-123","cmd":"stop_gc_profiling","thread_id":0}"#;
        let response = handle_command(input);
        assert!(response.contains(r#""ok":false"#));
        assert!(response.contains("dependent action"));
    }
}
