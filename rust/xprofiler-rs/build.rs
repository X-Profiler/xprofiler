//! Build script for xprofiler-rs
//!
//! This script configures the build process for the NAPI bindings
//! and handles platform-specific compilation requirements.

use std::env;
use std::path::PathBuf;

fn main() {
  // Configure NAPI build
  napi_build::setup();

  // Print cargo rerun-if-changed directives
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/");
  println!("cargo:rerun-if-changed=Cargo.toml");

  // Get target information
  let target = env::var("TARGET").unwrap();
  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

  println!("cargo:rustc-env=TARGET={}", target);
  println!("cargo:rustc-env=TARGET_OS={}", target_os);
  println!("cargo:rustc-env=TARGET_ARCH={}", target_arch);

  // Platform-specific configuration
  match target_os.as_str() {
    "linux" => {
      configure_linux();
    }
    "macos" => {
      configure_macos();
    }
    "windows" => {
      configure_windows();
    }
    _ => {
      println!("cargo:warning=Unsupported target OS: {}", target_os);
    }
  }

  // Architecture-specific configuration
  match target_arch.as_str() {
    "x86_64" => {
      println!("cargo:rustc-cfg=arch_x86_64");
    }
    "aarch64" => {
      println!("cargo:rustc-cfg=arch_aarch64");
    }
    "arm" => {
      println!("cargo:rustc-cfg=arch_arm");
    }
    _ => {
      println!(
        "cargo:warning=Unsupported target architecture: {}",
        target_arch
      );
    }
  }

  // Configure optimization flags
  configure_optimization();

  // Configure feature flags
  configure_features();

  // Generate version information
  generate_version_info();
}

/// Configure Linux-specific build settings
fn configure_linux() {
  println!("cargo:rustc-cfg=target_linux");

  // Link against required system libraries
  println!("cargo:rustc-link-lib=pthread");
  println!("cargo:rustc-link-lib=dl");
  println!("cargo:rustc-link-lib=m");

  // Set RPATH for shared libraries
  println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

  // Enable position-independent code
  println!("cargo:rustc-link-arg=-fPIC");
}

/// Configure macOS-specific build settings
fn configure_macos() {
  println!("cargo:rustc-cfg=target_macos");

  // Link against required system frameworks
  println!("cargo:rustc-link-lib=framework=CoreFoundation");
  println!("cargo:rustc-link-lib=framework=Security");
  println!("cargo:rustc-link-lib=framework=SystemConfiguration");

  // Set install name for dynamic library
  println!("cargo:rustc-link-arg=-Wl,-install_name,@rpath/libxprofiler_rs.dylib");

  // Enable position-independent code
  println!("cargo:rustc-link-arg=-fPIC");

  // Set minimum macOS version
  println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15");
}

/// Configure Windows-specific build settings
fn configure_windows() {
  println!("cargo:rustc-cfg=target_windows");

  // Link against required system libraries
  println!("cargo:rustc-link-lib=kernel32");
  println!("cargo:rustc-link-lib=user32");
  println!("cargo:rustc-link-lib=advapi32");
  println!("cargo:rustc-link-lib=ws2_32");
  println!("cargo:rustc-link-lib=psapi");
  println!("cargo:rustc-link-lib=pdh");

  // Set Windows subsystem
  println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE");
}

/// Configure optimization settings
fn configure_optimization() {
  let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

  match profile.as_str() {
    "release" => {
      println!("cargo:rustc-cfg=optimized");

      // Enable link-time optimization
      println!("cargo:rustc-link-arg=-flto");

      // Strip debug symbols in release builds
      #[cfg(target_os = "linux")]
      println!("cargo:rustc-link-arg=-Wl,--strip-debug");

      #[cfg(target_os = "macos")]
      println!("cargo:rustc-link-arg=-Wl,-dead_strip");

      // Optimize for size
      println!("cargo:rustc-env=CARGO_CFG_OPTIMIZE=size");
    }
    "debug" => {
      println!("cargo:rustc-cfg=debug_build");

      // Include debug information
      println!("cargo:rustc-link-arg=-g");

      // Debug assertions are automatically handled by Rust compiler
    }
    _ => {
      println!("cargo:warning=Unknown profile: {}", profile);
    }
  }
}

/// Configure feature flags based on enabled features
fn configure_features() {
  // Check for custom features
  if env::var("CARGO_FEATURE_BENCHMARKS").is_ok() {
    println!("cargo:rustc-cfg=feature_benchmarks");
  }

  if env::var("CARGO_FEATURE_TESTING").is_ok() {
    println!("cargo:rustc-cfg=feature_testing");
  }

  // Configure logging level based on build type
  let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
  match profile.as_str() {
    "release" => {
      println!("cargo:rustc-env=RUST_LOG=info");
    }
    "debug" => {
      println!("cargo:rustc-env=RUST_LOG=debug");
    }
    _ => {
      println!("cargo:rustc-env=RUST_LOG=warn");
    }
  }
}

/// Generate version and build information
fn generate_version_info() {
  // Get version from Cargo.toml
  let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
  println!("cargo:rustc-env=XPROFILER_VERSION={}", version);

  // Get build timestamp
  let build_time = chrono::Utc::now()
    .format("%Y-%m-%d %H:%M:%S UTC")
    .to_string();
  println!("cargo:rustc-env=XPROFILER_BUILD_TIME={}", build_time);

  // Get git commit hash if available
  if let Ok(output) = std::process::Command::new("git")
    .args(&["rev-parse", "--short", "HEAD"])
    .output()
  {
    if output.status.success() {
      let commit_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
      println!("cargo:rustc-env=XPROFILER_COMMIT_HASH={}", commit_hash);
    }
  }

  // Get git branch if available
  if let Ok(output) = std::process::Command::new("git")
    .args(&["rev-parse", "--abbrev-ref", "HEAD"])
    .output()
  {
    if output.status.success() {
      let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
      println!("cargo:rustc-env=XPROFILER_BRANCH={}", branch);
    }
  }

  // Generate build info module
  generate_build_info_module();
}

/// Generate a build info module with version and build information
fn generate_build_info_module() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = PathBuf::from(&out_dir).join("build_info.rs");

  let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
  let build_time = chrono::Utc::now()
    .format("%Y-%m-%d %H:%M:%S UTC")
    .to_string();
  let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
  let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

  let mut commit_hash = "unknown".to_string();
  let mut branch = "unknown".to_string();

  // Try to get git information
  if let Ok(output) = std::process::Command::new("git")
    .args(&["rev-parse", "--short", "HEAD"])
    .output()
  {
    if output.status.success() {
      commit_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
  }

  if let Ok(output) = std::process::Command::new("git")
    .args(&["rev-parse", "--abbrev-ref", "HEAD"])
    .output()
  {
    if output.status.success() {
      branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
  }

  let build_info_content = format!(
    r#"//! Build information generated at compile time

/// Version of the xprofiler-rs library
pub const VERSION: &str = "{}";

/// Build timestamp
pub const BUILD_TIME: &str = "{}";

/// Target triple
pub const TARGET: &str = "{}";

/// Build profile (debug/release)
pub const PROFILE: &str = "{}";

/// Git commit hash
pub const COMMIT_HASH: &str = "{}";

/// Git branch
pub const BRANCH: &str = "{}";

/// Full version string with build information
pub const FULL_VERSION: &str = concat!(
    "xprofiler-rs v", "{}",
    " (", "{}", "@", "{}", ")",
    " built on ", "{}",
    " for ", "{}",
    " in ", "{}", " mode"
);

/// Build information structure
#[derive(Debug, Clone)]
pub struct BuildInfo {{
    pub version: &'static str,
    pub build_time: &'static str,
    pub target: &'static str,
    pub profile: &'static str,
    pub commit_hash: &'static str,
    pub branch: &'static str,
    pub full_version: &'static str,
}}

impl BuildInfo {{
    /// Get build information
    pub const fn new() -> Self {{
        Self {{
            version: VERSION,
            build_time: BUILD_TIME,
            target: TARGET,
            profile: PROFILE,
            commit_hash: COMMIT_HASH,
            branch: BRANCH,
            full_version: FULL_VERSION,
        }}
    }}
}}

impl Default for BuildInfo {{
    fn default() -> Self {{
        Self::new()
    }}
}}
"#,
    version,
    build_time,
    target,
    profile,
    commit_hash,
    branch,
    version,
    branch,
    commit_hash,
    build_time,
    target,
    profile
  );

  std::fs::write(&dest_path, build_info_content).expect("Failed to write build info module");

  println!("cargo:rerun-if-changed={}", dest_path.display());
}
