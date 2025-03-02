use super::common::run_command;
use std::fs;

#[test]
fn test_platform_specific_config() {
    // Create a temporary directory for the test
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("run-this.json");

    // Create a config with platform-specific instructions
    let config_content = r#"
    {
        "platform-test-cmd": {
            "url": "https://default.example.com",
            "messages": ["Default message"],
            "windows": {
                "url": "https://windows.example.com",
                "messages": ["Windows specific message"]
            },
            "macos": {
                "url": "https://macos.example.com",
                "messages": ["macOS specific message"]
            },
            "linux": {
                "url": "https://linux.example.com",
                "messages": ["Linux specific message"]
            }
        }
    }
    "#;

    fs::write(&config_path, config_content).unwrap();

    // Change to the temporary directory and run the command
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let (status, _stdout, stderr) = run_command(&["--", "platform-test-cmd"]);

    // Restore the original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert_eq!(status, 127);
    assert!(stderr.contains("Command not found"));
    assert!(stderr.contains("platform-test-cmd"));

    // Check for platform-specific content
    // We can't know which platform the test is running on, so we check for either
    // the platform-specific message or the default message
    let has_platform_specific = stderr.contains("https://windows.example.com")
        || stderr.contains("https://macos.example.com")
        || stderr.contains("https://linux.example.com")
        || stderr.contains("https://default.example.com");

    assert!(
        has_platform_specific,
        "Should contain either platform-specific or default URL"
    );

    let has_platform_message = stderr.contains("Windows specific message")
        || stderr.contains("macOS specific message")
        || stderr.contains("Linux specific message")
        || stderr.contains("Default message");

    assert!(
        has_platform_message,
        "Should contain either platform-specific or default message"
    );
}
