// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use autopilot::mouse::Button;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers};
extern crate autopilot;

struct DelayState {
    delay: u64,
    enabled: bool,
}

async fn setup_hotkey(app: AppHandle) {
    // A shortcut using Alt + Shift + A to open the main window of the app
    let shortcut_delay =
        tauri_plugin_global_shortcut::Shortcut::new(Some(Modifiers::SUPER), Code::KeyA);
    // The shortcut ID
    let shortcut_delay_id = shortcut_delay.id();
    // Only works on Desktop, register the plugin here
    #[cfg(desktop)]
    app.app_handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::with_handler(move |app, key| {
                println!("key");
                let state = app.state::<Mutex<DelayState>>();
                let mut lock = state.lock().unwrap();
                if key.id() == shortcut_delay_id {
                    lock.enabled = !lock.enabled;
                }
                if !lock.enabled {
                    return;
                }
                let handle = app.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    let mut enabled: bool = true;
                    let mut delay: u64 = 1;
                    while enabled {
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                        let state = handle.state::<Mutex<DelayState>>();
                        let lock = state.lock().unwrap();
                        enabled = lock.enabled;
                        delay = lock.delay;
                        autopilot::mouse::click(Button::Left, Some(0));
                        println!("click");
                    }
                    println!("exiting");
                });
            })
            .build(),
        )
        .unwrap();
    app.app_handle()
        .global_shortcut()
        .register(shortcut_delay)
        .unwrap();
}

#[tauri::command]
async fn set_delay(app: AppHandle, delay: u64) {
    println!("2");
    let state = app.state::<Mutex<DelayState>>();
    let mut lock = state.lock().unwrap();
    lock.delay = delay;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(DelayState {
            delay: 0,
            enabled: false,
        }))
        .invoke_handler(tauri::generate_handler![set_delay])
        .setup(|app| {
            tauri::async_runtime::block_on(setup_hotkey(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_delay])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
