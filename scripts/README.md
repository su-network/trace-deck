# Scripts layout

- `scripts/windows/context-menu.ps1` — install/uninstall/quickstart for Windows context menu (requires Administrator, PowerShell ExecutionPolicy Bypass, built binary or quickstart).
- `scripts/windows/context-menu.bat` — cmd wrapper that delegates to the PowerShell script.
- `scripts/unix/build.sh` — Linux/macOS build helper (requires curl + Rust toolchain).
- `scripts/unix/cleanup.sh` — removes top-level *.md/*.txt files in repo root (use with care).

Usage examples:
- Windows install: `powershell -ExecutionPolicy Bypass -File scripts/windows/context-menu.ps1 -Action install`
- Windows quickstart: `powershell -ExecutionPolicy Bypass -File scripts/windows/context-menu.ps1 -Action quickstart`
- Linux/macOS build: `bash scripts/unix/build.sh`
- Linux/macOS cleanup: `bash scripts/unix/cleanup.sh`
