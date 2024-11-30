@echo off

REM Check if the executables exist
if not exist "target\release\multiclient_broadunicast.exe" (
    echo Building project first...
    cargo build --release
)

REM Run the main program
.\target\release\multiclient_broadunicast.exe