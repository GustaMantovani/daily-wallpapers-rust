@echo off
if not exist external_builds\windows (
    mkdir external_builds\windows
)

C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe /out:external_builds\windows\WallpaperChanger.exe src\windows\WallpaperChanger.cs

if %errorlevel% neq 0 (
    echo Compilation failed!
    exit /b %errorlevel%
)

echo Compilation succeeded!
