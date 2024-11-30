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
        powershell -Command "Invoke-WebRequest -Uri 'https://curl.se/windows/dl-7.80.0/curl-7.80.0-win64-mingw.zip' -OutFile 'curl.zip'"
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
set "INSTALL_DIR=%USERPROFILE%\bin"
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy the executables
echo Installing executables...
copy "target\release\multiclient-broadunicast.exe" "%INSTALL_DIR%\"
copy "target\release\server.exe" "%INSTALL_DIR%\"
copy "target\release\client.exe" "%INSTALL_DIR%\"

REM Add to PATH if not already there
echo %PATH% | find "%INSTALL_DIR%" >nul
if %ERRORLEVEL% NEQ 0 (
    setx PATH "%PATH%;%INSTALL_DIR%"
    set "PATH=%PATH%;%INSTALL_DIR%"
    echo Added executables to PATH
)

echo Installation complete!
echo You can now run the chat system with: multiclient-broadunicast OR by running run.bat
echo Please restart your terminal if this is a new installation.
pause 