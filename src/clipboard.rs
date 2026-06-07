use anyhow::{bail, Result};
use std::env;
use std::process::Command;

pub enum DisplayServer {
    Wayland,
    X11,
}

pub fn detect_display_server() -> DisplayServer {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        DisplayServer::Wayland
    } else {
        DisplayServer::X11
    }
}

pub fn check_dependencies() -> Result<()> {
    match detect_display_server() {
        DisplayServer::Wayland => {
            if Command::new("which").arg("wl-copy").output().map(|o| o.status.success()).unwrap_or(false) {
                Ok(())
            } else {
                bail!("wl-copy not found. Install wl-clipboard: sudo pacman -S wl-clipboard")
            }
        }
        DisplayServer::X11 => {
            let has_xclip = Command::new("which").arg("xclip").output().map(|o| o.status.success()).unwrap_or(false);
            let has_xsel = Command::new("which").arg("xsel").output().map(|o| o.status.success()).unwrap_or(false);

            if has_xclip || has_xsel {
                Ok(())
            } else {
                bail!("xclip or xsel not found. Install one: sudo pacman -S xclip")
            }
        }
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    match detect_display_server() {
        DisplayServer::Wayland => {
            let status = Command::new("wl-copy")
                .arg(text)
                .status()?;
            if !status.success() {
                bail!("wl-copy failed");
            }
        }
        DisplayServer::X11 => {
            let has_xclip = Command::new("which").arg("xclip").output().map(|o| o.status.success()).unwrap_or(false);

            if has_xclip {
                let status = Command::new("xclip")
                    .args(["-selection", "clipboard"])
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        use std::io::Write;
                        if let Some(stdin) = child.stdin.as_mut() {
                            stdin.write_all(text.as_bytes())?;
                        }
                        child.wait()
                    })?;
                if !status.success() {
                    bail!("xclip failed");
                }
            } else {
                let status = Command::new("xsel")
                    .args(["--clipboard", "--input"])
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        use std::io::Write;
                        if let Some(stdin) = child.stdin.as_mut() {
                            stdin.write_all(text.as_bytes())?;
                        }
                        child.wait()
                    })?;
                if !status.success() {
                    bail!("xsel failed");
                }
            }
        }
    }
    Ok(())
}
