Param(
    [ValidateSet("install", "uninstall", "quickstart", "help")]
    [string]$Action = "help"
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptDir "..\..")
$binaryPath = Join-Path $repoRoot "target\release\trace-deck.exe"
$releaseDir = Split-Path -Parent $binaryPath

function Write-Section($text) {
    Write-Host "-- $text" -ForegroundColor Cyan
}

function Require-Admin {
    $isAdmin = [bool]([Security.Principal.WindowsIdentity]::GetCurrent().Groups -match "S-1-5-32-544")
    if (-not $isAdmin) {
        throw "Administrator privileges required. Right-click PowerShell and choose 'Run as administrator'."
    }
}

function Ensure-Binary {
    if (-not (Test-Path $binaryPath)) {
        throw "Binary not found at $binaryPath. Run with -Action quickstart or build via 'cargo build --release'."
    }
}

function New-RegEntry {
    param(
        [string]$Path,
        [string]$Name,
        [string]$Value,
        [string]$Type = "String"
    )

    if (-not (Test-Path $Path)) {
        New-Item -Path $Path -Force | Out-Null
    }
    Set-ItemProperty -Path $Path -Name $Name -Value $Value -Type $Type -Force | Out-Null
}

function Remove-RegEntry {
    param([string]$Path)
    if (Test-Path $Path) {
        Remove-Item -Path $Path -Force -Recurse | Out-Null
    }
}

function Install-ContextMenu {
    Require-Admin
    Ensure-Binary

    Write-Section "Registering file types"
    New-RegEntry "HKLM:\SOFTWARE\Classes\.pdf" "" "PDFFile"
    New-RegEntry "HKLM:\SOFTWARE\Classes\.docx" "" "Word.Document.12"
    New-RegEntry "HKLM:\SOFTWARE\Classes\.jpg" "" "jpegfile"
    New-RegEntry "HKLM:\SOFTWARE\Classes\.jpeg" "" "jpegfile"
    New-RegEntry "HKLM:\SOFTWARE\Classes\.png" "" "pngfile"
    New-RegEntry "HKLM:\SOFTWARE\Classes\.gif" "" "giffile"
    New-RegEntry "HKLM:\SOFTWARE\Classes\.webp" "" "WebPImageFile"

    Write-Section "Registering application"
    New-RegEntry "HKLM:\SOFTWARE\Classes\Applications\trace-deck.exe" "" "trace-deck Document Analyzer"
    New-RegEntry "HKLM:\SOFTWARE\Classes\Applications\trace-deck.exe\shell\open\command" "" "`"$binaryPath`" process `"%1`""

    Write-Section "Adding context menus"
    New-RegEntry "HKLM:\SOFTWARE\Classes\PDFFile\shell\trace-deck" "" "Analyze with trace-deck"
    New-RegEntry "HKLM:\SOFTWARE\Classes\PDFFile\shell\trace-deck" "Icon" "$binaryPath,0"
    New-RegEntry "HKLM:\SOFTWARE\Classes\PDFFile\shell\trace-deck\command" "" "`"$binaryPath`" process `"%1`""

    New-RegEntry "HKLM:\SOFTWARE\Classes\Word.Document.12\shell\trace-deck" "" "Analyze with trace-deck"
    New-RegEntry "HKLM:\SOFTWARE\Classes\Word.Document.12\shell\trace-deck\command" "" "`"$binaryPath`" process `"%1`""

    @("jpegfile", "pngfile", "giffile", "WebPImageFile") | ForEach-Object {
        New-RegEntry "HKLM:\SOFTWARE\Classes\$_\shell\trace-deck" "" "Analyze with trace-deck"
        New-RegEntry "HKLM:\SOFTWARE\Classes\$_\shell\trace-deck\command" "" "`"$binaryPath`" process `"%1`""
    }

    Write-Section "Adding release folder to PATH"
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
    if ($currentPath -notlike "*$releaseDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$releaseDir;$currentPath", "Machine")
        Write-Host "Added $releaseDir to PATH" -ForegroundColor Green
    } else {
        Write-Host "Already in PATH" -ForegroundColor Yellow
    }

    Write-Section "Creating desktop shortcut"
    $desktopPath = [Environment]::GetFolderPath("Desktop")
    $shortcutPath = Join-Path $desktopPath "trace-deck.lnk"
    $ws = New-Object -ComObject WScript.Shell
    $shortcut = $ws.CreateShortcut($shortcutPath)
    $shortcut.TargetPath = $binaryPath
    $shortcut.Arguments = "info"
    $shortcut.WorkingDirectory = $repoRoot
    $shortcut.IconLocation = "$binaryPath,0"
    $shortcut.Save()

    Write-Host "Install completed" -ForegroundColor Green
}

function Uninstall-ContextMenu {
    Require-Admin

    Write-Section "Removing context menus"
    Remove-RegEntry "HKLM:\SOFTWARE\Classes\PDFFile\shell\trace-deck"
    Remove-RegEntry "HKLM:\SOFTWARE\Classes\Word.Document.12\shell\trace-deck"
    @("jpegfile", "pngfile", "giffile", "WebPImageFile") | ForEach-Object {
        Remove-RegEntry "HKLM:\SOFTWARE\Classes\$_\shell\trace-deck"
    }

    Write-Section "Removing application registration"
    Remove-RegEntry "HKLM:\SOFTWARE\Classes\Applications\trace-deck.exe"

    Write-Section "Removing desktop shortcut"
    $desktopPath = [Environment]::GetFolderPath("Desktop")
    $shortcutPath = Join-Path $desktopPath "trace-deck.lnk"
    if (Test-Path $shortcutPath) {
        Remove-Item -Path $shortcutPath -Force | Out-Null
    }

    Write-Host "Uninstall completed" -ForegroundColor Green
}

function Quickstart {
    Write-Section "Building release binary"
    $build = Start-Process -FilePath "cargo" -ArgumentList "build", "--release" -WorkingDirectory $repoRoot -Wait -PassThru
    if ($build.ExitCode -ne 0) {
        throw "cargo build failed with exit code $($build.ExitCode)"
    }
    Install-ContextMenu
}

function Show-Help {
    Write-Host "trace-deck Windows helper" -ForegroundColor Cyan
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File context-menu.ps1 -Action <install|uninstall|quickstart|help>" -ForegroundColor Gray
    Write-Host "  install    Add context menu entries and shortcut" -ForegroundColor Gray
    Write-Host "  uninstall  Remove context menu entries and shortcut" -ForegroundColor Gray
    Write-Host "  quickstart Build release binary then install context menu" -ForegroundColor Gray
    Write-Host "  help       Show this help" -ForegroundColor Gray
}

try {
    switch ($Action) {
        "install"    { Install-ContextMenu }
        "uninstall"  { Uninstall-ContextMenu }
        "quickstart" { Quickstart }
        default      { Show-Help }
    }
    exit 0
} catch {
    Write-Host "[ERROR] $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
