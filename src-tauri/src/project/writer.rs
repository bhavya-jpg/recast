use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use crate::project::ProjectMetadata;

pub struct ProjectWriteRequest {
    pub output_path: PathBuf,
    pub metadata: ProjectMetadata,
    pub recording_path: PathBuf,
    pub cursor_path: PathBuf,
    pub audio_path: PathBuf,
    pub edits_json: String,
}

pub fn write_project(request: ProjectWriteRequest) -> Result<PathBuf> {
    let file = File::create(&request.output_path)?;
    let mut writer = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);

    writer.start_file("metadata.json", options)?;
    writer.write_all(serde_json::to_string_pretty(&request.metadata)?.as_bytes())?;

    writer.start_file("cursor.json", options)?;
    copy_file(&request.cursor_path, &mut writer)?;

    writer.start_file("audio.wav", options)?;
    copy_file(&request.audio_path, &mut writer)?;

    writer.start_file("edits.json", options)?;
    writer.write_all(request.edits_json.as_bytes())?;

    writer.start_file("recording.mp4", options)?;
    copy_file(&request.recording_path, &mut writer)?;

    writer.finish()?;
    Ok(request.output_path)
}

fn copy_file(path: &Path, writer: &mut ZipWriter<File>) -> Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        writer.write_all(&buffer[..read])?;
    }
    Ok(())
}
