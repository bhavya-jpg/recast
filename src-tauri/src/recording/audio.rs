use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::Result;

pub struct AudioCapture {
    path: PathBuf,
    started_at: Instant,
    sample_rate: u32,
    channels: u16,
}

impl AudioCapture {
    pub fn new(path: PathBuf, started_at: Instant) -> Self {
        Self {
            path,
            started_at,
            sample_rate: 48_000,
            channels: 2,
        }
    }

    pub fn finish(self) -> Result<PathBuf> {
        let duration_secs = self.started_at.elapsed().as_secs_f64();
        write_silence_wav(
            &self.path,
            self.sample_rate,
            self.channels,
            duration_secs.max(0.0),
        )?;
        Ok(self.path)
    }
}

fn write_silence_wav(
    path: &Path,
    sample_rate: u32,
    channels: u16,
    duration_secs: f64,
) -> Result<()> {
    let bytes_per_sample = 2u16;
    let total_samples = (duration_secs * sample_rate as f64).round() as u32;
    let data_len = total_samples * channels as u32 * bytes_per_sample as u32;

    let mut file = File::create(path)?;
    file.write_all(b"RIFF")?;
    file.write_all(&(36 + data_len).to_le_bytes())?;
    file.write_all(b"WAVE")?;
    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&channels.to_le_bytes())?;
    file.write_all(&sample_rate.to_le_bytes())?;
    let byte_rate = sample_rate * channels as u32 * bytes_per_sample as u32;
    file.write_all(&byte_rate.to_le_bytes())?;
    let block_align = channels * bytes_per_sample;
    file.write_all(&block_align.to_le_bytes())?;
    file.write_all(&(bytes_per_sample * 8).to_le_bytes())?;
    file.write_all(b"data")?;
    file.write_all(&data_len.to_le_bytes())?;
    file.seek(SeekFrom::Start(44))?;
    file.write_all(&vec![0u8; data_len as usize])?;
    Ok(())
}
