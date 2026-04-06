// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod watcher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(watcher::WatcherState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_active_ports,
            commands::get_available_states,
            commands::watch_port,
            commands::unwatch_port,
            commands::get_watched_ports,
            commands::kill_process,
            commands::block_internet,
            commands::unblock_internet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


/*
.manage() it takes any value and stores it inside Tauri's app, making it available to all 
commands via State<'_, T>. This is where the single shared WatcherState instance is created - every 
command that ask for State<'_, WatcherState> gets a reference to this exact same object.
- tauri::generate_handler![] - is a rust macro that registers all the functions the frontend is allowd 
to call via invoke(). If a rust function is not listed here, calling iit from svelte will fail at run time
 */