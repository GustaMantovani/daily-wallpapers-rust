@echo off
REM Cria o diretório de saída, se não existir
if not exist external_builds\windows (
    mkdir external_builds\windows
)

REM Compila o arquivo C# e gera o executável
csc /out:external_builds\windows\WallpaperChanger.exe src\windows\WallpaperChanger.cs

REM Verifica se a compilação foi bem-sucedida
if %errorlevel% neq 0 (
    echo Compilation failed!
    exit /b %errorlevel%
)

echo Compilation succeeded!
