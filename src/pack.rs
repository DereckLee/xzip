use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::codec::EncodingKind;
use crate::error::XzipError;
use crate::filter::PathFilter;

pub fn pack_path(
    input: &Path,
    output_zip: &Path,
    encoding: EncodingKind,
    recursive: bool,
    include: &[String],
    exclude: &[String],
) -> Result<(), XzipError> {
    if !input.exists() {
        return Err(XzipError::InvalidInput(input.to_path_buf()));
    }
    let filter = PathFilter::new(include, exclude)?;

    let out_file = File::create(output_zip)?;
    let mut writer = ZipWriter::new(out_file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    if input.is_file() {
        let base = input
            .file_name()
            .and_then(|x| x.to_str())
            .ok_or_else(|| XzipError::InvalidInput(input.to_path_buf()))?;
        add_file(&mut writer, input, Path::new(base), options, encoding)?;
    } else {
        for entry in WalkDir::new(input) {
            let entry = entry.map_err(|e| std::io::Error::other(e.to_string()))?;
            let path = entry.path();
            if path == input {
                continue;
            }
            if !recursive && path.parent() != Some(input) {
                continue;
            }
            let relative = path
                .strip_prefix(input)
                .map_err(|_| XzipError::InvalidInput(input.to_path_buf()))?;
            let zip_path = normalize_zip_path(relative);
            if !filter.allows(&zip_path) {
                continue;
            }
            if entry.file_type().is_dir() {
                // Keep directory structure stable in tools that expect explicit entries.
                writer.add_directory(&zip_path, options)?;
            } else if entry.file_type().is_file() {
                add_file(&mut writer, path, Path::new(&zip_path), options, encoding)?;
            }
        }
    }

    writer.finish()?;
    Ok(())
}

fn add_file<W: Write + std::io::Seek>(
    writer: &mut ZipWriter<W>,
    src_path: &Path,
    zip_relative_path: &Path,
    options: SimpleFileOptions,
    encoding: EncodingKind,
) -> Result<(), XzipError> {
    let display_name = normalize_zip_path(zip_relative_path);
    // Validate representability under chosen encoding to keep behavior explicit.
    let encoded = encoding.encode(&display_name)?;
    let normalized_name = encoding.decode(&encoded)?;

    writer.start_file(normalized_name, options)?;
    let mut input_file = File::open(src_path)?;
    let mut buf = Vec::new();
    input_file.read_to_end(&mut buf)?;
    writer.write_all(&buf)?;
    Ok(())
}

fn normalize_zip_path(path: &Path) -> String {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        normalized.push(component);
    }
    normalized.to_string_lossy().replace('\\', "/")
}
