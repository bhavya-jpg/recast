use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use zip::ZipArchive;

use crate::project::ProjectMetadata;

#[derive(Debug, Clone)]
pub struct ProjectOpenResult {
    pub metadata: ProjectMetadata,
    pub recording_path: PathBuf,
    pub cursor_path: PathBuf,
    pub edits_path: PathBuf,
}

pub fn open_project(path: &Path) -> Result<ProjectOpenResult> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    let metadata: ProjectMetadata = {
        let mut metadata_entry = archive.by_name("metadata.json")?;
        let mut bytes = Vec::new();
        metadata_entry.read_to_end(&mut bytes)?;
        serde_json::from_slice(&bytes)?
    };

    let cache_dir = cache_dir_for(path)?;
    fs::create_dir_all(&cache_dir)?;

    let recording_path = extract_entry(&mut archive, "recording.mp4", &cache_dir.join("recording.mp4"))?;
    let cursor_path = extract_entry(&mut archive, "cursor.json", &cache_dir.join("cursor.json"))?;
    let edits_path = extract_entry(&mut archive, "edits.json", &cache_dir.join("edits.json"))?;

    Ok(ProjectOpenResult {
        metadata,
        recording_path,
        cursor_path,
        edits_path,
    })
}

fn cache_dir_for(project_path: &Path) -> Result<PathBuf> {
    let metadata = fs::metadata(project_path)?;
    let stem = project_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("project");
    Ok(env::temp_dir().join("recast-cache").join(format!("{stem}-{}", metadata.len())))
}

fn extract_entry(archive: &mut ZipArchive<File>, name: &str, path: &Path) -> Result<PathBuf> {
    let mut entry = archive.by_name(name).with_context(|| format!("missing {name} in project"))?;
    let mut output = File::create(path)?;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = entry.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        output.write_all(&buffer[..read])?;
    }
    Ok(path.to_path_buf())
}
