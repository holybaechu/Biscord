// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_handler, AppHandle, Manager, Window, WindowBuilder};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

#[tauri::command]
fn close(app_handle: AppHandle) {
    app_handle.save_window_state(StateFlags::all()).unwrap();
    app_handle.get_window("main").unwrap().close().unwrap();
}

#[tauri::command]
fn minimize(window: Window) {
    window.minimize().unwrap()
}

#[tauri::command]
fn maximize(window: Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap()
    } else {
        window.maximize().unwrap()
    }
}

struct Mod {
    script: &'static str,
    style: &'static str
}

static MODS: phf::Map<&'static str, Mod> = phf::phf_map! {
    "Shelter" => Mod {
        script: "https://raw.githubusercontent.com/uwu/shelter-builds/main/shelter.js",
        style: ""
    },
    "Vencord" => Mod {
        script: "https://raw.githubusercontent.com/Vencord/builds/main/browser.js",
        style: "https://raw.githubusercontent.com/Vencord/builds/main/browser.css"
    }
};

fn get_mods_js() -> String {
    let mut s = String::new();
    for mod_ in MODS.values() {
        if mod_.script.is_empty() { continue; }
        s.push_str(reqwest::blocking::get(mod_.script).unwrap().text().unwrap().as_str());
        s.push_str(";")
    }
    s
}

#[tauri::command]
fn get_mods_css() -> String {
    let mut s = String::new();
    for mod_ in MODS.values() {
        if mod_.style.is_empty() { continue; }
        s.push_str(reqwest::blocking::get(mod_.style).unwrap().text().unwrap().as_str());
        s.push_str("\n")
    }
    s
}

#[tauri::command]
fn loaded(app_handle: AppHandle) {
    app_handle.get_window("splash").unwrap().close().unwrap();
    app_handle.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn splash_loaded(app_handle: AppHandle) {
    app_handle.get_window("splash").unwrap().show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().with_state_flags(StateFlags::union(StateFlags::union(StateFlags::POSITION, StateFlags::SIZE), StateFlags::MAXIMIZED)).with_denylist(&[&"splash"]).build())
        .invoke_handler(generate_handler![
            close,
            minimize,
            maximize,
            get_mods_css,
            loaded,
            splash_loaded
        ])
        
        .setup(move |app| {
            let splash = WindowBuilder::new(app, "splash", tauri::WindowUrl::default())
                .decorations(false)
                .resizable(false)
                .visible(false)
                .center()
                .inner_size(300.0, 350.0)
                .title("Discord")
                .build()?;

            window_shadows::set_shadow(&splash, true).unwrap();

            let win = WindowBuilder::new(app, "main", tauri::WindowUrl::External("https://discord.com/app".parse().unwrap()))
            .initialization_script(
                &format!(
                    "{};{};",
                    get_mods_js(),
                    include_str!("../preinject.js")
                )
            )
            .decorations(false)
            .visible(false)
            .center()
            .inner_size(1280.0, 720.0)
            .title("Discord")
            .build()?;

            win.set_decorations(false).unwrap();
            window_shadows::set_shadow(&win, true).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
