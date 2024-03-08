// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::{SystemTime, UNIX_EPOCH};

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn menu_handle(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app_handle.get_window("main").unwrap();
            if !window.is_visible().unwrap() {
                window.show().unwrap();
            }
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "show" => {
                let window = app_handle.get_window("main").unwrap();
                if !window.is_visible().unwrap() {
                    window.show().unwrap();
                }
                window.set_focus().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

fn main() {
    // 设置一天后退出
    // let time = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_millis();
    // // println!("{}", time);
    // if time > 1709719982000 {
    //     return;
    // }

    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let show = CustomMenuItem::new("show".to_string(), "显示主界面");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| menu_handle(app, event))
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // event.window().hide().unwrap();
                #[cfg(not(target_os = "macos"))]
                {
                    event.window().hide().unwrap();
                }

                #[cfg(target_os = "macos")]
                {
                    tauri::AppHandle::hide(&event.window().app_handle()).unwrap();
                }
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
