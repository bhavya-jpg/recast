#[cfg(windows)]
mod windows;

#[cfg(not(windows))]
mod fallback;

#[cfg(windows)]
pub use windows::PlatformAudioSession;

#[cfg(not(windows))]
pub use fallback::PlatformAudioSession;
