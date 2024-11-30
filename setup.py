import os
import subprocess
import sys
import platform

def install_rust():
    """Install Rust if not already installed"""
    try:
        subprocess.run(["rustc", "--version"], capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("Rust not found. Installing Rust...")
        if platform.system() == "Windows":
            # Download rustup-init.exe
            subprocess.run([
                "powershell", "-Command", 
                "Invoke-WebRequest -Uri 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' -OutFile 'rustup-init.exe'"
            ], check=True)
            subprocess.run(["rustup-init.exe", "-y"], check=True)
        elif platform.system() in ["Linux", "Darwin"]:
            subprocess.run([
                "curl", "--proto", "=https", "--tlsv1.2", "-sSf", 
                "https://sh.rustup.rs", "-o", "rustup-init.sh"
            ], check=True