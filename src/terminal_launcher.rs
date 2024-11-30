use std::path::{Path, PathBuf};
use std::process::Command;

pub fn launch_terminal(title: &str, command: &str) -> std::io::Result<()> {
    let current_exe = std::env::current_exe()?;
    let project_dir = current_exe.parent().unwrap_or_else(|| Path::new("."));

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg(title)
            .arg(command)
            .current_dir(project_dir)
            .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        // Check desktop environment and use appropriate terminal
        let terminal = if std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().contains("KDE") {
            "konsole"
        } else {
            "gnome-terminal"
        };

        Command::new(terminal)
            .arg("--title")
            .arg(title)
            .arg("--")
            .arg("sh")
            .arg("-c")
            .arg(format!("{} ; read", command))
            .current_dir(project_dir)
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "tell application \"Terminal\"
    activate
    do script \"cd {} && {}\"
end tell",
                project_dir.display(),
                command
            ))
            .spawn()?;
    }

    Ok(())
}

pub fn get_project_root() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}