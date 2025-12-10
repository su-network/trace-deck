@echo off
REM trace-deck Windows helper wrapper
REM Usage: context-menu.bat install|uninstall|quickstart|help

set ACTION=%1
if "%ACTION%"=="" set ACTION=help

powershell -ExecutionPolicy Bypass -File "%~dp0context-menu.ps1" -Action %ACTION%
set EXITCODE=%ERRORLEVEL%
exit /b %EXITCODE%
