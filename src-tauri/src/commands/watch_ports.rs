use crate::watcher::WatcherState;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::time::Duration;
use tauri::State;
use tokio::time::sleep;

#[tauri::command]
pub async fn watch_port(
    port: u16,
    command_line: String,
    state: State<'_, WatcherState>,
) -> Result<(), String> {
    let mut watchers = state.watchers.lock().map_err(|e| e.to_string())?;

    if watchers.contains_key(&port) {
        return Err(format!("Already watching port {}", port));
    }

    let task = tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;

            let is_alive = check_port_alive(port);

            if !is_alive {
                let parts: Vec<&str> = command_line.splitn(2, ' ').collect();
                let program = parts[0];
                let args = parts.get(1).unwrap_or(&"");

                let _ = std::process::Command::new(program)
                    .args(args.split_whitespace())
                    .spawn();
            }
        }
    });
    watchers.insert(port, task.abort_handle());
    Ok(())
}

#[tauri::command]
pub async fn unwatch_port(port: u16, state: State<'_, WatcherState>) -> Result<(), String> {
    let mut watchers = state.watchers.lock().map_err(|e| e.to_string())?;

    match watchers.remove(&port) {
        Some(handle) => {
            handle.abort();
            Ok(())
        }
        None => Err(format!("Not watching port {}", port)),
    }
}

#[tauri::command]
pub async fn get_watched_ports(
    state: State<'_, WatcherState>,
) -> Result<Vec<u16>, String> {
    let watchers = state.watchers.lock().map_err(|e| e.to_string())?;
    Ok(watchers.keys().cloned().collect())
}

fn check_port_alive(port: u16) -> bool {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    match get_sockets_info(af_flags, proto_flags) {
        Ok(sockets) => sockets.iter().any(|s| match &s.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp) => tcp.local_port == port,
            ProtocolSocketInfo::Udp(udp) => udp.local_port == port,
        }),
        Err(_) => false,
    }
}

/*
- pub async fn - the async keyword means this function can do non-blocking work. Tuair requires
commands that use State to be async.
- State<'_, WatcherState> - This is how tauri injects the shared state into your command. The '_ is
a lifetime (Rust making sure the reference stays valid)
- .lock().map_err(|e| e.to_string())? - .lock() opens the Mutex (entes the room) .map_err()  converts
the error type so rust is happy. The ? means :if this fials, return the error immediately.
- tokio::spawn(async move {loop { ... }) - spawn a background taks. move means the task takes ownership of port and command_line.
Loop runs forever until aborted.
- sleep(Duration::from_secs(5)).await; - waits 5 seconds wihout blocking ohter tasks. The .await is how async Rust "pauses" a task.
- splitn(2, ' ') -  splits the command strin into at most 2 parts: the program name and everything else. e.g. "bun run dev" -> ["bun", "run dev"]
- task.abort_handle() - gets the remote control for the task, which we soter in the hashmap
- check _port_alive - a helper that re-scans all sockets and returns true if any socket uses that port number.
- watcher.remove(&port) - removes the entry from the hasmap and returns the value that was there (Some(handle) or None if the key didn't exist)
- match ... { Some(handle) => .. None => ... } - this is Rust's patter metching. It's like a switch but exhaustive - you must handle both cases or it won't compile.
- handle.abort() - sends a cacellation sgnal to the background loop task. The task stops at its nex .await point (the sleep).
- watchers.keys() - returns all the keys (port numbers) in the hashmap
- .copied() u16  - is a simple number type, so we can just copu the values out instead of dealing with references. 
. collect()  - turns the iterator into a Vec<u16>
 */
