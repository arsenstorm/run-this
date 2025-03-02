#!/usr/bin/env bash
set -e

# Colors for terminal output
BOLD="\033[1m"
RESET="\033[0m"
RED="\033[31m"
GREEN="\033[32m"
YELLOW="\033[33m"
BLUE="\033[34m"
CYAN="\033[36m"

# Default installation directory
INSTALL_DIR="$HOME/.run-this"
BIN_DIR="$INSTALL_DIR/bin"
EXEC_PATH="$BIN_DIR/run-this"

# GitHub repository information
REPO_OWNER="arsenstorm"
REPO_NAME="run-this"
GITHUB_URL="https://github.com/$REPO_OWNER/$REPO_NAME"
LATEST_RELEASE_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"

# Print banner
echo -e "${BOLD}${BLUE}run-this${RESET} installer"
echo -e "A utility that gracefully handles missing command dependencies"
echo ""

# Detect OS and architecture
detect_os_arch() {
  local os="$(uname -s)"
  local arch="$(uname -m)"
  
  case "$os" in
    "Darwin")
      OS="macos"
      ;;
    "Linux")
      OS="linux"
      ;;
    "MINGW"*|"MSYS"*|"CYGWIN"*)
      OS="windows"
      ;;
    *)
      echo -e "${RED}Unsupported operating system: $os${RESET}"
      exit 1
      ;;
  esac
  
  case "$arch" in
    "x86_64"|"amd64")
      ARCH="x86_64"
      ;;
    "arm64"|"aarch64")
      ARCH="aarch64"
      ;;
    *)
      echo -e "${RED}Unsupported architecture: $arch${RESET}"
      exit 1
      ;;
  esac
  
  echo -e "Detected ${CYAN}$OS${RESET} on ${CYAN}$ARCH${RESET}"
}

# Get the latest release version and download URL
get_latest_release() {
  echo -e "Fetching latest release information..."
  
  if command -v curl &> /dev/null; then
    RELEASE_INFO=$(curl -s "$LATEST_RELEASE_URL")
  elif command -v wget &> /dev/null; then
    RELEASE_INFO=$(wget -qO- "$LATEST_RELEASE_URL")
  else
    echo -e "${RED}Error: Neither curl nor wget is installed${RESET}"
    exit 1
  fi
  
  VERSION=$(echo "$RELEASE_INFO" | grep -o '"tag_name": *"[^"]*"' | cut -d'"' -f4)
  
  if [ -z "$VERSION" ]; then
    echo -e "${RED}Error: Could not determine the latest version${RESET}"
    exit 1
  fi
  
  # Construct asset name based on OS and architecture
  ASSET_NAME="run-this-$VERSION-$OS-$ARCH.tar.gz"
  
  # Find download URL for the asset
  DOWNLOAD_URL=$(echo "$RELEASE_INFO" | grep -o "\"browser_download_url\": *\"[^\"]*$ASSET_NAME\"" | cut -d'"' -f4)
  
  if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Error: Could not find download URL for $ASSET_NAME${RESET}"
    exit 1
  fi
  
  echo -e "Latest version: ${GREEN}$VERSION${RESET}"
}

# Download and install the binary
install_binary() {
  echo -e "Installing run-this..."
  
  # Create installation directory
  mkdir -p "$BIN_DIR"
  
  # Download the binary
  echo -e "Downloading from ${CYAN}$DOWNLOAD_URL${RESET}"
  
  if command -v curl &> /dev/null; then
    curl -L "$DOWNLOAD_URL" -o "/tmp/$ASSET_NAME"
  elif command -v wget &> /dev/null; then
    wget -O "/tmp/$ASSET_NAME" "$DOWNLOAD_URL"
  fi
  
  # Extract the binary
  echo -e "Extracting..."
  tar -xzf "/tmp/$ASSET_NAME" -C "$BIN_DIR"
  rm "/tmp/$ASSET_NAME"
  
  # Make the binary executable
  chmod +x "$EXEC_PATH"
  
  echo -e "${GREEN}Successfully installed run-this to $EXEC_PATH${RESET}"
}

# Add to PATH if needed
setup_path() {
  # Check if BIN_DIR is already in PATH
  if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo -e "Adding run-this to your PATH..."
    
    # Detect shell
    SHELL_NAME="$(basename "$SHELL")"
    
    case "$SHELL_NAME" in
      "bash")
        SHELL_PROFILE="$HOME/.bashrc"
        ;;
      "zsh")
        SHELL_PROFILE="$HOME/.zshrc"
        ;;
      "fish")
        SHELL_PROFILE="$HOME/.config/fish/config.fish"
        ;;
      *)
        echo -e "${YELLOW}Warning: Unknown shell $SHELL_NAME. You'll need to manually add $BIN_DIR to your PATH.${RESET}"
        return
        ;;
    esac
    
    if [ -f "$SHELL_PROFILE" ]; then
      if [ "$SHELL_NAME" = "fish" ]; then
        echo "fish_add_path $BIN_DIR" >> "$SHELL_PROFILE"
      else
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_PROFILE"
      fi
      
      echo -e "${GREEN}Added run-this to PATH in $SHELL_PROFILE${RESET}"
      echo -e "Restart your terminal or run 'source $SHELL_PROFILE' to use run-this"
    else
      echo -e "${YELLOW}Warning: Shell profile not found at $SHELL_PROFILE. You'll need to manually add $BIN_DIR to your PATH.${RESET}"
    fi
  else
    echo -e "${GREEN}run-this is already in your PATH${RESET}"
  fi
}

# Run the installation
detect_os_arch
get_latest_release
install_binary
setup_path

echo ""
echo -e "${GREEN}${BOLD}run-this has been successfully installed!${RESET}"
echo -e "Run ${CYAN}run-this --help${RESET} to get started"
echo -e "Visit ${BLUE}$GITHUB_URL${RESET} for more information" 