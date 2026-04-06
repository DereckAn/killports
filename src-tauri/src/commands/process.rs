use sysinfo::System;

#[tauri::command]
pub fn kill_process(pid: u32) -> Result<(), String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    match sys.process(sysinfo::Pid::from_u32(pid)) {
        Some(process) => {
            if process.kill() {
                Ok(())
            } else {
                Err(format!("Failed to kill process with PID {}", pid))
            }
        }
        None => Err(format!("Process with PID {} not found", pid)),
    }
}

#[tauri::command]
pub fn block_internet(pid: u32) -> Result<(), String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let process = sys
        .process(sysinfo::Pid::from_u32(pid))
        .ok_or_else(|| format!("No process found with PID {}", pid))?;
    let exe_path = process
        .exe()
        .ok_or_else(|| "Could not determine executable path".to_string())?
        .to_string_lossy()
        .into_owned();
    if exe_path.is_empty() {
        return Err("Could not determine executable path".to_string());
    }
    add_firewall_rule(&format!("killports-block-out-{}", pid), "out", &exe_path)?;
    add_firewall_rule(&format!("killports-block-in-{}", pid), "in", &exe_path)?;
    Ok(())
}

#[tauri::command]
pub fn unblock_internet(pid: u32) -> Result<(), String> {
    delete_firewall_rule(&format!("killports-block-out-{}", pid));
    delete_firewall_rule(&format!("killports-block-in-{}", pid));
    Ok(())
}

fn add_firewall_rule(name: &str, dir: &str, exe: &str) -> Result<(), String> {
    let status = std::process::Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "add",
            "rule",
            &format!("name={}", name),
            &format!("dir={}", dir),
            "action=block",
            &format!("program={}", exe),
            "enable=yes",
        ])
        .status()
        .map_err(|e| format!("Failed to execute netsh: {}", e))?;
    if status.success() {
        Ok(())
    } else {
        Err("Failed to add firewall rule — run killports as administrator.".to_string())
    }
}

fn delete_firewall_rule(name: &str) {
    let _ = std::process::Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "delete",
            "rule",
            &format!("name={}", name),
        ])
        .status();
}

/*
- #[tauri::command] - Rust attribute that tells Tauri "this function can be called from the frontend via invoke()."
- let mut sys = System::new_all(); - creates a new snapshot of the OS. The mut neans we're going to modify it.

*/
