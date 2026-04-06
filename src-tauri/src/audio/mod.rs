mod platform;
pub mod wav;

use std::path::PathBuf;

use anyhow::Result;

/// Configuration for audio capture.
#[derive(Debug, Clone)]
pub struct AudioCaptureConfig {
    /// Path to write the WAV output file.
    pub output_path: PathBuf,
    /// Capture system/loopback audio.
    pub capture_loopback: bool,
    /// Capture microphone audio (future: will be mixed or stored as separate track).
    pub capture_microphone: bool,
}

/// Handle to a running audio capture session.
/// The capture runs on a background thread and writes PCM data to a WAV file.
/// Call `stop()` to finalize the WAV file and get the output path.
pub struct AudioCaptureSession {
    inner: platform::PlatformAudioSession,
}

impl AudioCaptureSession {
    /// Start a new audio capture session with the given configuration.
    pub fn start(config: AudioCaptureConfig) -> Result<Self> {
        let inner = platform::PlatformAudioSession::start(config)?;
        Ok(Self { inner })
    }

    /// Stop the capture, finalize the WAV file, and return the output path.
    /// This blocks until the capture thread has finished writing.
    pub fn stop(self) -> Result<PathBuf> {
        self.inner.stop()
    }
}
