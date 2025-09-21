//! Unit tests for the xprofiler-rs library.
//! 
//! This module contains basic unit tests for core components.

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_functionality() {
        // Basic test to ensure the library compiles
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_version_info() {
        let version = env!("CARGO_PKG_VERSION");
        assert!(!version.is_empty());
    }
}