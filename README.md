# `run-this`

`run-this` is a utility command that gracefully handles missing command dependencies.

## Example

```bash
# This environment doesn't have bun.sh installed.
run-this -- bun run dev
```

Since this environment doesn't have Bun installed, the command `bun run dev` will not run and would normally throw an error.

`run-this` gracefully stops the system from throwing this error and informs the user that they do not have Bun installed, along with instructions on how to install it.

## Installation

### Using the Installation Script (Recommended)

#### Unix (macOS, Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/arsenstorm/run-this/main/install.sh | bash
```

#### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/arsenstorm/run-this/main/install.ps1 | iex
```

### From Source

```bash
# Clone the repository
git clone https://github.com/arsenstorm/run-this.git
cd run-this

# Build and install
cargo install --path .
```

### Using Cargo

```bash
cargo install run-this
```

## Usage

```bash
run-this -- <command> [arguments]
```

The `--` is used to separate the `run-this` command from the command you want to run. This is necessary if the command you want to run has options that might be interpreted by `run-this`.

## Custom Installation Instructions

You can create a `run-this.json` file in your project directory to provide custom installation instructions for specific commands.

When a command is not found, `run-this` will check this file for installation instructions before falling back to built-in suggestions.

The tool automatically detects the user's operating system and provides platform-specific installation instructions when available.

```json
{
  "flutter": {
    "url": "https://flutter.dev/docs/get-started/install",
    "messages": ["Or use Homebrew on macOS: brew install --cask flutter"],
    "windows": {
      "url": "https://flutter.dev/docs/get-started/install/windows",
      "messages": ["Or use Chocolatey: choco install flutter"]
    },
    "macos": {
      "url": "https://flutter.dev/docs/get-started/install/macos",
      "messages": ["Or use Homebrew: brew install --cask flutter"]
    },
    "linux": {
      "url": "https://flutter.dev/docs/get-started/install/linux",
      "messages": ["Or use snapd: sudo snap install flutter --classic"]
    }
  },
  "deno": {
    "url": "https://deno.land/#installation",
    "windows": {
      "messages": ["PowerShell: irm https://deno.land/install.ps1 | iex"]
    },
    "macos": {
      "messages": ["Use Homebrew: brew install deno"]
    }
  }
}
```

Supported platforms:

- Windows (`windows`)
- MacOS (`macos`)
- Linux (`linux`)

## Testing

The project includes both unit tests and integration tests organized in the `src/tests` directory:

### Unit Tests

Unit tests cover the core functionality of the application, including:

- Platform detection
- Configuration parsing
- Platform-specific configuration handling

### Integration Tests

Integration tests verify the CLI functionality:

- Command execution
- Error handling
- Custom configuration loading
- Platform-specific behavior

To run all tests:

```bash
cargo test
```

## License

MIT
