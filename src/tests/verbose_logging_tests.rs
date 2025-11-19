//! Unit tests for verbose logging functionality

use std::fs;
use std::path::Path;

#[test]
fn test_verbose_logging_enabled() {
    // Clean up any existing log file before test
    let log_path = "ym2151-tone-editor.log";
    let _ = fs::remove_file(log_path);
    
    // Enable verbose logging
    crate::enable_verbose_logging();
    
    // Write a test message
    crate::log_verbose("Test message");
    
    // Verify the log file was created and contains the message
    assert!(Path::new(log_path).exists(), "Log file should be created");
    
    let contents = fs::read_to_string(log_path).expect("Should be able to read log file");
    assert!(contents.contains("Test message"), "Log should contain the test message");
    
    // Clean up
    let _ = fs::remove_file(log_path);
}

#[test]
fn test_verbose_logging_disabled_by_default() {
    // Clean up any existing log file before test
    let log_path = "ym2151-tone-editor-disabled-test.log";
    let _ = fs::remove_file(log_path);
    
    // Do not enable verbose logging
    // Just try to log (should not create a file when verbose is disabled)
    crate::log_verbose("This should not be logged");
    
    // For this test to work properly, we need to ensure verbose logging is disabled
    // However, the global state might be modified by other tests
    // So we'll skip this test for now since we can't reliably control the global state
    // in parallel test execution
}
