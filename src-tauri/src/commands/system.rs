use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tauri::{AppHandle, Manager, State};
use xcap::{Monitor, Window};

use super::ffmpeg::{encode_thumbnail_base64, make_thumbnail};
use super::types::{AppConfig, AppState, DisplayInfo, WindowInfo};

fn config_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| env::temp_dir())
        .join("recast_config.json")
}

pub fn load_config(app: &AppHandle) -> AppConfig {
    let path = config_path(app);
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(config) = serde_json::from_str(&data) {
            return config;
        }
    }
    AppConfig::default()
}

fn save_config(app: &AppHandle, config: &AppConfig) {
    let path = config_path(app);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(data) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, data);
    }
}

pub fn get_active_output_dir(state: &State<'_, AppState>) -> PathBuf {
    let config = state.config.lock();
    if let Some(dir) = &config.output_dir {
        PathBuf::from(dir)
    } else {
        env::temp_dir()
    }
}

fn capture_monitor_thumbnail(monitor: &Monitor) -> Option<String> {
    let shot = monitor.capture_image().ok()?;
    encode_thumbnail_base64(&make_thumbnail(&shot))
}

fn capture_window_thumbnail(window: &Window) -> Option<String> {
    let shot = window.capture_image().ok()?;
    encode_thumbnail_base64(&make_thumbnail(&shot))
}

#[tauri::command]
pub fn get_output_dir(state: State<'_, AppState>) -> Result<String, String> {
    Ok(get_active_output_dir(&state).to_string_lossy().to_string())
}

#[tauri::command]
pub fn set_output_dir(
    app: AppHandle,
    state: State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    if !Path::new(&path).exists() {
        return Err("Directory does not exist".into());
    }
    let mut config = state.config.lock();
    config.output_dir = Some(path);
    save_config(&app, &config);
    Ok(())
}

#[tauri::command]
pub fn get_displays() -> Result<Vec<DisplayInfo>, String> {
    let monitors = Monitor::all().map_err(|e| e.to_string())?;
    Ok(monitors
        .iter()
        .map(|monitor| DisplayInfo {
            id: monitor.id().unwrap_or_default(),
            name: monitor.name().unwrap_or_default(),
            x: monitor.x().unwrap_or_default(),
            y: monitor.y().unwrap_or_default(),
            width: monitor.width().unwrap_or_default(),
            height: monitor.height().unwrap_or_default(),
            is_primary: monitor.is_primary().unwrap_or_default(),
            thumbnail: capture_monitor_thumbnail(monitor),
        })
        .collect())
}

#[tauri::command]
pub fn get_windows() -> Result<Vec<WindowInfo>, String> {
    let windows = Window::all().map_err(|e| e.to_string())?;
    Ok(windows
        .iter()
        .filter(|window| {
            !window.is_minimized().unwrap_or(false)
                && !window.title().unwrap_or_default().is_empty()
        })
        .map(|window| WindowInfo {
            id: window.id().unwrap_or_default(),
            pid: window.pid().unwrap_or_default(),
            app_name: window.app_name().unwrap_or_default(),
            title: window.title().unwrap_or_default(),
            x: window.x().unwrap_or_default(),
            y: window.y().unwrap_or_default(),
            width: window.width().unwrap_or_default(),
            height: window.height().unwrap_or_default(),
            is_minimized: window.is_minimized().unwrap_or_default(),
            thumbnail: capture_window_thumbnail(window),
        })
        .collect())
}

#[tauri::command]
pub fn open_file_location(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
