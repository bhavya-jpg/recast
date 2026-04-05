use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorSample {
    pub timestamp_us: u64,
    pub x: i32,
    pub y: i32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub visible: bool,
    pub left_down: bool,
    pub right_down: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorClickEvent {
    pub timestamp_us: u64,
    pub button: String,
    pub phase: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CursorTrack {
    pub samples: Vec<CursorSample>,
    pub clicks: Vec<CursorClickEvent>,
}

#[derive(Debug, Clone, Copy)]
struct CursorState {
    x: i32,
    y: i32,
    visible: bool,
    left_down: bool,
    right_down: bool,
}

pub fn spawn_cursor_capture(
    stop_flag: Arc<AtomicBool>,
    clock: Instant,
) -> Result<thread::JoinHandle<CursorTrack>> {
    thread::Builder::new()
        .name("recast-cursor".into())
        .spawn(move || {
            let mut track = CursorTrack::default();
            let mut previous: Option<(CursorState, u64)> = None;

            while !stop_flag.load(Ordering::Acquire) {
                let now_us = clock.elapsed().as_micros() as u64;
                if let Some(current) = sample_cursor_state() {
                    let (velocity_x, velocity_y) = previous
                        .map(|(prev, prev_ts)| {
                            let delta_t =
                                ((now_us.saturating_sub(prev_ts)).max(1)) as f32 / 1_000_000.0;
                            (
                                (current.x - prev.x) as f32 / delta_t,
                                (current.y - prev.y) as f32 / delta_t,
                            )
                        })
                        .unwrap_or((0.0, 0.0));

                    if let Some((prev, _)) = previous {
                        if prev.left_down != current.left_down {
                            track.clicks.push(CursorClickEvent {
                                timestamp_us: now_us,
                                button: "left".into(),
                                phase: if current.left_down { "down" } else { "up" }.into(),
                                x: current.x,
                                y: current.y,
                            });
                        }
                        if prev.right_down != current.right_down {
                            track.clicks.push(CursorClickEvent {
                                timestamp_us: now_us,
                                button: "right".into(),
                                phase: if current.right_down { "down" } else { "up" }.into(),
                                x: current.x,
                                y: current.y,
                            });
                        }
                    }

                    track.samples.push(CursorSample {
                        timestamp_us: now_us,
                        x: current.x,
                        y: current.y,
                        velocity_x,
                        velocity_y,
                        visible: current.visible,
                        left_down: current.left_down,
                        right_down: current.right_down,
                    });
                    previous = Some((current, now_us));
                }

                thread::sleep(Duration::from_millis(8));
            }

            track
        })
        .map_err(Into::into)
}

pub fn write_cursor_track(path: &Path, track: &CursorTrack) -> Result<()> {
    std::fs::write(path, serde_json::to_vec_pretty(track)?)?;
    Ok(())
}

#[cfg(windows)]
fn sample_cursor_state() -> Option<CursorState> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        CURSOR_SHOWING, CURSORINFO, GetCursorInfo, GetCursorPos,
    };

    let mut point = POINT::default();
    let mut info = CURSORINFO {
        cbSize: std::mem::size_of::<CURSORINFO>() as u32,
        ..Default::default()
    };

    unsafe {
        if GetCursorPos(&mut point).is_err() {
            return None;
        }
        if GetCursorInfo(&mut info).is_err() {
            return None;
        }
    }

    Some(CursorState {
        x: point.x,
        y: point.y,
        visible: info.flags == CURSOR_SHOWING,
        left_down: unsafe { (GetAsyncKeyState(VK_LBUTTON.0 as i32) as u16 & 0x8000) != 0 },
        right_down: unsafe { (GetAsyncKeyState(VK_RBUTTON.0 as i32) as u16 & 0x8000) != 0 },
    })
}

#[cfg(not(windows))]
fn sample_cursor_state() -> Option<CursorState> {
    None
}
