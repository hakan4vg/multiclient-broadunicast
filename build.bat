@echo off

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Rust is not installed. Installing Rust...
    curl --proto '=https' --tlsv1.2 -sSf https://win.rustup.rs > rustup-init.exe
    rustup-init.exe -y
    del rustup-init.exe
    
    REM Refresh environment variables
    call "%USERPROFILE%\.cargo\env"
    
    echo Rust installation complete!
)

REM Build the project in release mode
echo Building project...
cargo build --release

REM Create directory for executables if it doesn't exist
if not exist "%USERPROFILE%\bin" mkdir "%USERPROFILE%\bin"

REM Copy the executables
echo Installing executables...
copy "target\release\multiclient-broadunicast.exe" "%USERPROFILE%\bin\"
copy "target\release\server.exe" "%USERPROFILE%\bin\"
copy "target\release\client.exe" "%USERPROFILE%\bin\"

REM Add to PATH if not already there
echo %PATH% | find "%USERPROFILE%\bin" >nul
if %ERRORLEVEL% NEQ 0 (
    setx PATH "%PATH%;%USERPROFILE%\bin"
    echo Added executables to PATH
)

echo Installation complete!
echo You can now run the chat system with: multiclient-broadunicast OR by running run.bat
pause 