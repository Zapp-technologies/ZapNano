use std::process::{Command, Stdio};
use std::io::Write;

pub fn get_clipboard() -> Result<String, String> {
    if cfg!(target_os = "windows") {
        let output = Command::new("powershell")
            .args(&["-NoProfile", "-Command", "Get-Clipboard"])
            .output()
            .map_err(|e| e.to_string())?;
        
        let s = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(s.trim_end().to_string())
    } else {
        let output = Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                return Ok(String::from_utf8_lossy(&out.stdout).to_string());
            }
        }
        
        let output = Command::new("xsel")
            .args(&["--output", "--clipboard"])
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                return Ok(String::from_utf8_lossy(&out.stdout).to_string());
            }
        }
        
        let output = Command::new("wl-paste")
            .args(&["-n"])
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                return Ok(String::from_utf8_lossy(&out.stdout).to_string());
            }
        }
        
        Err("No clipboard utility found (xclip, xsel, or wl-paste)".to_string())
    }
}

pub fn set_clipboard(text: &str) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        let mut child = Command::new("powershell")
            .args(&["-NoProfile", "-Command", "Set-Clipboard", "-Value", "$Input"])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
        }
        
        child.wait().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        let child = Command::new("xclip")
            .args(&["-selection", "clipboard", "-i"])
            .stdin(Stdio::piped())
            .spawn();
        if let Ok(mut c) = child {
            if let Some(mut stdin) = c.stdin.take() {
                if stdin.write_all(text.as_bytes()).is_ok() {
                    drop(stdin);
                    if c.wait().is_ok() {
                        return Ok(());
                    }
                }
            }
        }
        
        let child = Command::new("xsel")
            .args(&["--input", "--clipboard"])
            .stdin(Stdio::piped())
            .spawn();
        if let Ok(mut c) = child {
            if let Some(mut stdin) = c.stdin.take() {
                if stdin.write_all(text.as_bytes()).is_ok() {
                    drop(stdin);
                    if c.wait().is_ok() {
                        return Ok(());
                    }
                }
            }
        }
        
        let child = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn();
        if let Ok(mut c) = child {
            if let Some(mut stdin) = c.stdin.take() {
                if stdin.write_all(text.as_bytes()).is_ok() {
                    drop(stdin);
                    if c.wait().is_ok() {
                        return Ok(());
                    }
                }
            }
        }
        
        Err("No clipboard utility found (xclip, xsel, or wl-copy)".to_string())
    }
}
