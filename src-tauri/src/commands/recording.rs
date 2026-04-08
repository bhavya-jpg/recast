use std::fs;

use tauri::State;

use super::ffmpeg::probe_video_metadata;
use super::system::get_active_output_dir;
use super::types::{AppState, RecordingEntry};
use crate::project::writer::{ProjectWriteRequest, write_project};
use crate::project::{ProjectMetadata, ProjectVideoMetadata};
use crate::recording::{CaptureTarget, RecordingOptions};
use crate::render::graph::RenderState;

#[tauri::command]
pub fn start_recording(
    target_type: String,
    target_id: u32,
    options: Option<RecordingOptions>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let target = CaptureTarget::resolve(&target_type, target_id).map_err(|e| e.to_string())?;
    let output_dir = get_active_output_dir(&state);
    state
        .recording_manager
        .start(target, output_dir, options.unwrap_or_default())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_recording(state: State<'_, AppState>) -> Result<String, String> {
    let artifacts = state.recording_manager.stop().map_err(|e| e.to_string())?;
    let final_path = get_active_output_dir(&state).join(format!(
        "recast_recording_{}.recast",
        artifacts.started_at_unix_ms
    ));
    let recording_meta = probe_video_metadata(&artifacts.recording_path)?;
    let metadata = ProjectMetadata {
        schema_version: 1,
        created_at_unix_ms: artifacts.started_at_unix_ms,
        capture_target: artifacts.capture_target.clone(),
        stats: artifacts.stats.clone(),
        video: ProjectVideoMetadata {
            width: if recording_meta.width > 0 {
                recording_meta.width
            } else {
                artifacts.capture_target.crop.width
            },
            height: if recording_meta.height > 0 {
                recording_meta.height
            } else {
                artifacts.capture_target.crop.height
            },
            fps: recording_meta.fps.round().max(1.0) as u32,
            duration_ms: artifacts.stats.duration_ms,
        },
    };
    let default_render_state = RenderState {
        trim_end: artifacts.stats.duration_ms as f64 / 1000.0,
        ..RenderState::default()
    };
    let project_path = write_project(ProjectWriteRequest {
        output_path: final_path.clone(),
        metadata,
        recording_path: artifacts.recording_path.clone(),
        cursor_path: artifacts.cursor_path.clone(),
        audio_path: artifacts.audio_path.clone(),
        edits_json: serde_json::to_string_pretty(&default_render_state)
            .unwrap_or_else(|_| "{}".into()),
    })
    .map_err(|e| e.to_string())?;

    let _ = fs::remove_file(&artifacts.recording_path);
    let _ = fs::remove_file(&artifacts.cursor_path);
    let _ = fs::remove_file(&artifacts.audio_path);

    *state.last_file_path.lock() = Some(project_path.to_string_lossy().to_string());
    Ok(project_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn list_recordings(state: State<'_, AppState>) -> Result<Vec<RecordingEntry>, String> {
    let dir_path = get_active_output_dir(&state);
    let mut entries = Vec::new();

    for entry in fs::read_dir(&dir_path)
        .map_err(|e| e.to_string())?
        .flatten()
    {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if !matches!(extension, "recast" | "mp4") {
            continue;
        }

        if let Ok(meta) = entry.metadata() {
            let created = meta
                .modified()
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            entries.push(RecordingEntry {
                filename: name,
                path: path.to_string_lossy().to_string(),
                size_bytes: meta.len(),
                created,
            });
        }
    }

    entries.sort_by(|a, b| b.created.cmp(&a.created));
    Ok(entries)
}
