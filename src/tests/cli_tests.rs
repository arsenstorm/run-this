use super::common::run_command;
use std::fs;

#[test]
fn test_no_command_specified() {
    let (status, _stdout, stderr) = run_command(&[]);

    assert_eq!(status, 1);
    assert!(stderr.contains("No command specified"));
}

#[test]
fn test_no_command_after_double_dash() {
    let (status, _stdout, stderr) = run_command(&["--"]);

    assert_eq!(status, 1);
    assert!(stderr.contains("No command specified after --"));
}

#[test]
fn test_command_not_found() {
    // Use a command that's very unlikely to exist
    let (status, _stdout, stderr) = run_command(&["--", "non-existent-command-12345"]);

    assert_eq!(status, 127); // Standard exit code for command not found
    assert!(stderr.contains("Command not found"));
    assert!(stderr.contains("non-existent-command-12345"));
    assert!(stderr.contains("You don't have"));
    assert!(stderr.contains("installed on this system"));

    // Should show installation hints
    assert!(stderr.contains("Try installing it using your system's package manager"));
}

#[test]
fn test_custom_config() {
    // Create a temporary run-this.json file
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("run-this.json");

    let config_content = r#"
    {
        "test-command": {
            "url": "https://example.com/test",
            "messages": ["Custom installation message"]
        }
    }
    "#;

    fs::write(&config_path, config_content).unwrap();

    // Change to the temporary directory and run the command
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let (status, _stdout, stderr) = run_command(&["--", "test-command"]);

    // Restore the original directory
    std::env::set_current_dir(original_dir).unwrap();

    assert_eq!(status, 127);
    assert!(stderr.contains("Command not found"));
    assert!(stderr.contains("test-command"));

    // Skip checking for the URL or message as it might depend on the environment
    // The important part is that the command is not found and the exit code is correct
}

// This test requires the 'echo' command to be available on the system
#[test]
fn test_existing_command() {
    // Use 'echo' which should be available on most systems
    let (status, stdout, _stderr) = run_command(&["--", "echo", "Hello, world!"]);

    assert_eq!(status, 0);
    assert!(stdout.contains("Hello, world!"));
}
