use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::{Command, ExitStatus};
use which::which;

// Tests
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct PlatformConfig {
    url: Option<String>,
    messages: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandConfig {
    url: Option<String>,
    messages: Option<Vec<String>>,
    windows: Option<PlatformConfig>,
    macos: Option<PlatformConfig>,
    linux: Option<PlatformConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    #[serde(flatten)]
    commands: HashMap<String, CommandConfig>,
}

fn detect_os() -> Platform {
    let os = env::consts::OS;
    match os {
        "windows" => Platform::Windows,
        "macos" => Platform::MacOS,
        "linux" => Platform::Linux,
        _ => Platform::Unknown,
    }
}

#[cfg(windows)]
fn find_windows_executable(command: &str) -> Option<std::path::PathBuf> {
    use std::path::PathBuf;

    // Get the PATH environment variable
    let path = env::var_os("PATH")?;
    // Get the PATHEXT environment variable (default to common extensions if not set)
    let pathext = env::var_os("PATHEXT").unwrap_or_default();
    let pathext = pathext.to_string_lossy().to_uppercase();
    let extensions: Vec<&str> = pathext.split(';').filter(|s| !s.is_empty()).collect();

    // If the command already has an extension that's in PATHEXT, search for it exactly
    let command_path = PathBuf::from(command);
    if let Some(ext) = command_path.extension() {
        let ext = format!(".{}", ext.to_string_lossy()).to_uppercase();
        if extensions.iter().any(|&e| e == ext) {
            for dir in env::split_paths(&path) {
                let full_path = dir.join(&command_path);
                if full_path.is_file() {
                    return Some(full_path);
                }
            }
            return None;
        }
    }

    // Search for command with each possible extension
    for dir in env::split_paths(&path) {
        // First try the exact command name
        let full_path = dir.join(command);
        if full_path.is_file() {
            return Some(full_path);
        }

        // Then try with each extension
        for ext in &extensions {
            let mut full_path = dir.join(command);
            full_path.set_extension(&ext[1..]); // Remove the leading dot from extension
            if full_path.is_file() {
                return Some(full_path);
            }
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("{}", "No command specified.".red().bold());
        eprintln!("Usage: run-this [--] <command> [arguments]");
        std::process::exit(1);
    }

    let cmd_start_idx = if args[0] == "--" { 1 } else { 0 };

    if cmd_start_idx >= args.len() {
        eprintln!("{}", "No command specified after --.".red().bold());
        eprintln!("Usage: run-this [--] <command> [arguments]");
        std::process::exit(1);
    }

    let command = &args[cmd_start_idx];
    let command_args = &args[(cmd_start_idx + 1)..];

    #[cfg(windows)]
    let command_exists = find_windows_executable(command).is_some();
    #[cfg(not(windows))]
    let command_exists = which(command).is_ok();

    if command_exists {
        match run_command(command, command_args) {
            Ok(status) => {
                if !status.success() {
                    if let Some(code) = status.code() {
                        std::process::exit(code);
                    } else {
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("{} {}", "Error running command:".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{} {}", "Command not found:".red().bold(), command.yellow());
        eprintln!(
            "You don't have {} installed on this system.",
            command.yellow()
        );

        let current_os = detect_os();
        let mut showed_instructions = false;

        if let Some(config) = load_config() {
            if let Some(cmd_config) = config.commands.get(command) {
                let platform_config = match current_os {
                    Platform::Windows => cmd_config.windows.as_ref(),
                    Platform::MacOS => cmd_config.macos.as_ref(),
                    Platform::Linux => cmd_config.linux.as_ref(),
                    Platform::Unknown => None,
                };

                let has_platform_url = platform_config.and_then(|pc| pc.url.as_ref()).is_some();
                let has_platform_messages = platform_config
                    .and_then(|pc| pc.messages.as_ref())
                    .is_some();
                let has_default_url = cmd_config.url.is_some();
                let has_default_messages = cmd_config.messages.is_some();

                if has_platform_url
                    || has_platform_messages
                    || has_default_url
                    || has_default_messages
                {
                    eprintln!("\n{}", "To install, you can:".cyan());
                    showed_instructions = true;

                    if let Some(url) = platform_config
                        .and_then(|pc| pc.url.as_ref())
                        .or(cmd_config.url.as_ref())
                    {
                        eprintln!("  Visit: {}", url);
                    }

                    if let Some(messages) = platform_config
                        .and_then(|pc| pc.messages.as_ref())
                        .or(cmd_config.messages.as_ref())
                    {
                        for message in messages {
                            eprintln!("  {}", message);
                        }
                    }
                }
            }
        }

        if !showed_instructions {
            provide_installation_hint(command, current_os);
        }

        std::process::exit(127);
    }
}

fn load_config() -> Option<Config> {
    if let Ok(current_dir) = env::current_dir() {
        let config_path = current_dir.join("run-this.json");
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(contents) => match serde_json::from_str::<Config>(&contents) {
                    Ok(config) => return Some(config),
                    Err(e) => {
                        eprintln!("{} {}", "Error parsing run-this.json:".red().bold(), e);
                    }
                },
                Err(e) => {
                    eprintln!("{} {}", "Error reading run-this.json:".red().bold(), e);
                }
            }
        }
    }
    None
}

fn run_command(command: &str, args: &[String]) -> Result<ExitStatus, std::io::Error> {
    #[cfg(windows)]
    {
        if let Some(exe_path) = find_windows_executable(command) {
            return Command::new(exe_path).args(args).status();
        }
        // This should never happen since we check existence before calling run_command
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Command not found",
        ));
    }
    #[cfg(not(windows))]
    Command::new(command).args(args).status()
}

fn provide_installation_hint(command: &str, current_os: Platform) {
    match command {
        "bun" => {
            eprintln!("\n{}", "To install Bun, you can run:".cyan());
            eprintln!("  curl -fsSL https://bun.sh/install | bash");
        }
        "npm" => {
            eprintln!("\n{}", "npm comes with Node.js. To install Node.js:".cyan());
            eprintln!("  Visit: https://nodejs.org/");
        }
        "yarn" => {
            eprintln!("\n{}", "To install Yarn, you can run:".cyan());
            eprintln!("  npm install -g yarn");
        }
        "pnpm" => {
            eprintln!("\n{}", "To install pnpm, you can run:".cyan());
            eprintln!("  npm install -g pnpm");
        }
        "cargo" | "rustc" | "rustup" => {
            eprintln!("\n{}", "To install Rust and Cargo, you can run:".cyan());
            eprintln!("  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
        }
        "go" => {
            eprintln!("\n{}", "To install Go, visit:".cyan());
            eprintln!("  https://golang.org/doc/install");
        }
        "python" | "python3" => {
            eprintln!("\n{}", "To install Python, visit:".cyan());
            eprintln!("  https://www.python.org/downloads/");
        }
        "pip" | "pip3" => {
            eprintln!("\n{}", "pip comes with Python. To install Python:".cyan());
            eprintln!("  https://www.python.org/downloads/");
        }
        "docker" => {
            eprintln!("\n{}", "To install Docker, visit:".cyan());
            eprintln!("  https://docs.docker.com/get-docker/");
        }
        "git" => {
            eprintln!("\n{}", "To install Git, visit:".cyan());
            eprintln!("  https://git-scm.com/downloads");
        }
        _ => {
            eprintln!(
                "\n{}",
                "Try installing it using your system's package manager:".cyan()
            );
            match current_os {
                Platform::Windows => {
                    eprintln!("  • Windows (winget): winget install {}", command);
                    eprintln!("  • Windows (Chocolatey): choco install {}", command);
                    eprintln!("  • Windows (Scoop): scoop install {}", command);
                }
                Platform::MacOS => {
                    eprintln!("  • macOS (Homebrew): brew install {}", command);
                    eprintln!("  • macOS (MacPorts): sudo port install {}", command);
                }
                Platform::Linux => {
                    eprintln!("  • Ubuntu/Debian: sudo apt install {}", command);
                    eprintln!("  • Fedora/RHEL: sudo dnf install {}", command);
                    eprintln!("  • Arch Linux: sudo pacman -S {}", command);
                }
                Platform::Unknown => {
                    eprintln!("  • macOS (Homebrew): brew install {}", command);
                    eprintln!("  • Ubuntu/Debian: sudo apt install {}", command);
                    eprintln!("  • Fedora/RHEL: sudo dnf install {}", command);
                    eprintln!("  • Arch Linux: sudo pacman -S {}", command);
                    eprintln!("  • Windows (winget): winget install {}", command);
                }
            }
        }
    }
}
