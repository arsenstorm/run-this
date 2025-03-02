// Unit tests
mod unit_tests;

// Integration tests
mod cli_tests;
mod platform_tests;

// Common test utilities
#[cfg(test)]
pub mod common {
    use std::path::PathBuf;
    use std::process::Command;

    pub fn run_command(args: &[&str]) -> (i32, String, String) {
        let output = if cfg!(test) {
            // When running as a test, find the binary in the target directory
            let mut bin_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

            // Check if we're in debug or release mode
            if cfg!(debug_assertions) {
                bin_path.push("target/debug/run-this");
            } else {
                bin_path.push("target/release/run-this");
            }

            // On Windows, add .exe extension
            if cfg!(windows) {
                bin_path.set_extension("exe");
            }

            Command::new(bin_path)
                .args(args)
                .output()
                .expect("Failed to execute command")
        } else {
            // For manual testing, use the command in PATH
            Command::new("run-this")
                .args(args)
                .output()
                .expect("Failed to execute command")
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let status = output.status.code().unwrap_or(-1);

        (status, stdout, stderr)
    }
}
