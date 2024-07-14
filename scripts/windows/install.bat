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

:: Create the .dwr\bin directory in the user's home directory
set "DWR_DIR=%USERPROFILE%\.dwr\bin"
if not exist "%DWR_DIR%" (
    mkdir "%DWR_DIR%"
)

:: Move the executable to the .dwr\bin directory
set "EXECUTABLE_NAME=daily-wallpapers-rust.exe"
move "target\release\%EXECUTABLE_NAME%" "%DWR_DIR%"

:: Check if the move was successful
if %ERRORLEVEL% neq 0 (
    echo Failed to move the executable!
    exit /b %ERRORLEVEL%
)

echo Executable moved to %DWR_DIR%

:: Add .dwr\bin to the PATH permanently
setx PATH "%PATH%;%DWR_DIR%"
if %ERRORLEVEL% neq 0 (
    echo Failed to add %DWR_DIR% to PATH!
    exit /b %ERRORLEVEL%
)

echo %DWR_DIR% added to PATH permanently

:: Pause the script so you can see the output
pause
