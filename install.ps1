# run-this installer for Windows
# Similar to Bun's installation script

# GitHub repository information
$RepoOwner = "arsenstorm"
$RepoName = "run-this"
$GitHubUrl = "https://github.com/$RepoOwner/$RepoName"
$LatestReleaseUrl = "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"

# Installation directory
$InstallDir = "$env:USERPROFILE\.run-this"
$BinDir = "$InstallDir\bin"
$ExecPath = "$BinDir\run-this.exe"

# Print banner
Write-Host "`nrun-this installer" -ForegroundColor Blue
Write-Host "A utility that gracefully handles missing command dependencies`n"

# Detect architecture
function Get-Architecture {
    $arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
    
    if ($arch -eq "AMD64") {
        return "x86_64"
    }
    elseif ($arch -eq "ARM64") {
        return "aarch64"
    }
    else {
        Write-Host "Unsupported architecture: $arch" -ForegroundColor Red
        exit 1
    }
}

# Get the latest release version and download URL
function Get-LatestRelease {
    Write-Host "Fetching latest release information..."
    
    try {
        $releaseInfo = Invoke-RestMethod -Uri $LatestReleaseUrl -UseBasicParsing
        $version = $releaseInfo.tag_name
        
        if (-not $version) {
            Write-Host "Error: Could not determine the latest version" -ForegroundColor Red
            exit 1
        }
        
        # Construct asset name based on architecture
        $assetName = "run-this-$version-windows-$script:Arch.zip"
        
        # Find download URL for the asset
        $asset = $releaseInfo.assets | Where-Object { $_.name -eq $assetName }
        
        if (-not $asset) {
            Write-Host "Error: Could not find download URL for $assetName" -ForegroundColor Red
            exit 1
        }
        
        $script:DownloadUrl = $asset.browser_download_url
        $script:Version = $version
        
        Write-Host "Latest version: $version" -ForegroundColor Green
    }
    catch {
        Write-Host "Error fetching release information: $_" -ForegroundColor Red
        exit 1
    }
}

# Download and install the binary
function Install-Binary {
    Write-Host "Installing run-this..."
    
    # Create installation directory
    if (-not (Test-Path $BinDir)) {
        New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
    }
    
    # Download the binary
    Write-Host "Downloading from $script:DownloadUrl"
    
    # Ensure temp directory exists
    $tempDir = [System.IO.Path]::GetTempPath()
    if (-not (Test-Path $tempDir)) {
        Write-Host "Creating temporary directory..."
        New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    }
    
    $tempFile = Join-Path $tempDir $assetName
    
    try {
        Invoke-WebRequest -Uri $script:DownloadUrl -OutFile $tempFile -UseBasicParsing
        
        # Extract the binary
        Write-Host "Extracting..."
        Expand-Archive -Path $tempFile -DestinationPath $BinDir -Force
        Remove-Item $tempFile
        
        Write-Host "Successfully installed run-this to $ExecPath" -ForegroundColor Green
    }
    catch {
        Write-Host "Error downloading or extracting: $_" -ForegroundColor Red
        exit 1
    }
}

# Add to PATH if needed
function Update-Path {
    $userPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
    
    if (-not $userPath.Contains($BinDir)) {
        Write-Host "Adding run-this to your PATH..."
        
        try {
            $newPath = "$userPath;$BinDir"
            [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
            Write-Host "Added run-this to PATH in User environment variables" -ForegroundColor Green
            Write-Host "You may need to restart your terminal to use run-this"
        }
        catch {
            Write-Host "Warning: Could not add to PATH. You'll need to manually add $BinDir to your PATH." -ForegroundColor Yellow
        }
    }
    else {
        Write-Host "run-this is already in your PATH" -ForegroundColor Green
    }
}

# Run the installation
$Arch = Get-Architecture
Get-LatestRelease
Install-Binary
Update-Path

Write-Host "`nrun-this has been successfully installed!" -ForegroundColor Green
Write-Host "Run 'run-this --help' to get started"
Write-Host "Visit $GitHubUrl for more information" 