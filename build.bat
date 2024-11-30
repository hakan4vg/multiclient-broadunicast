@echo off
setlocal enabledelayedexpansion

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Rust is not installed. Installing Rust...
    
    REM Check if curl exists, if not download it
    where curl >nul 2>nul
    if %ERRORLEVEL% NEQ 0 (
        echo Downloading curl...
        powershell -Command "Invoke-WebRequest -Uri 'https://curl.se/windows/dl-8.11.0_4/curl-8.11.0_4-win64-mingw.zip' -OutFile 'curl.zip'"
        powershell -Command "Expand-Archive -Path 'curl.zip' -DestinationPath 'curl'"
        set "PATH=%CD%\curl\bin;%PATH%"
    )
    
    REM Download and run rustup-init
    echo Downloading Rust installer...
    curl --proto =https --tlsv1.2 -sSf https://win.rustup.rs > rustup-init.exe
    
    echo Installing Rust...
    rustup-init.exe -y --default-toolchain stable
    del rustup-init.exe
    
    REM Refresh environment variables
    set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
)

REM Build the project in release mode
echo Building project...
cargo build --release

REM Create directory for executables if it doesn't exist
set "INSTALL_DIR=%USERPROFILE%\Desktop\HakanServer"
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy the executables
echo Copying executables to Desktop\HakanServer...
copy "target\release\multiclient_broadunicast.exe" "%INSTALL_DIR%\"
copy "target\release\server.exe" "%INSTALL_DIR%\"
copy "target\release\client.exe" "%INSTALL_DIR%\"


echo Build complete!
echo You can now run the chat system with: Desktop/HakanServer/multiclient_broadunicast OR by running run.bat
pause 