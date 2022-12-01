#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::time::Duration;
use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayEvent, SystemTray, RunEvent, SystemTrayMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    
    std::thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = SineWave::new(1_000_000.0)
            .take_duration(Duration::from_secs_f32(1.0))
            .amplify(0.001)
            .repeat_infinite();

        sink.append(source);

        sink.sleep_until_end();
    });
    
    let tray = SystemTray::new().with_menu(tray_menu);
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(tray)
        .on_system_tray_event(move |_app, event| if let SystemTrayEvent::MenuItemClick { id, .. } = event {
            if id.as_str() == "quit" {
                std::process::exit(0);
            }
        })
        .build(tauri::generate_context!("tauri.conf.json"))
        .expect("error while building tauri application");
        
        app.run(|_app_handle, event| if let RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        });
}
