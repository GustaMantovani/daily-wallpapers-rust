@echo off

:: Build the Rust project
echo Building the Rust project...
cargo build --release

:: Check if the build was successful
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    exit /b %ERRORLEVEL%
)

echo Build succeeded!

echo Building external dependencies...
.\scripts\windows\build_wallpaper_changer.bat

:: Check if the build of external dependencies was successful
if %ERRORLEVEL% neq 0 (
    echo Build dependencies failed!
    exit /b %ERRORLEVEL%
)

echo Build dependencies succeeded!

:: Create the .dwr directory in the user's home directory
set "DWR_DIR=%USERPROFILE%\.dwr"
if not exist "%DWR_DIR%" (
    mkdir "%DWR_DIR%"
)

:: Move the external_builds folder to the target directory
set "TARGET_DIR=%DWR_DIR%"
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%"
)
move /Y "%~dp0external_builds" "%TARGET_DIR%"
if %ERRORLEVEL% neq 0 (
    echo Failed to move external_builds!
    exit /b %ERRORLEVEL%
)

echo external_builds moved successfully!

:: Move the executable to the target directory
set "EXECUTABLE_NAME=target\release\daily-wallpapers-rust.exe"
set "TARGET_EXEC_DIR=%DWR_DIR%"
if not exist "%TARGET_EXEC_DIR%" (
    mkdir "%TARGET_EXEC_DIR%"
)
move /Y "%~dp0%EXECUTABLE_NAME%" "%TARGET_EXEC_DIR%"
if %ERRORLEVEL% neq 0 (
    echo Failed to move %EXECUTABLE_NAME%!
    exit /b %ERRORLEVEL%
)

echo %EXECUTABLE_NAME% moved successfully!

:: Pause the script so you can see the output
pause
