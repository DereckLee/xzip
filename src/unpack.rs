use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};

use zip::ZipArchive;

use crate::codec::EncodingKind;
use crate::error::RzipError;
use crate::filter::PathFilter;

pub fn unpack_archive(
    input_zip: &Path,
    output_dir: &Path,
    encoding: EncodingKind,
    include: &[String],
    exclude: &[String],
) -> Result<(), RzipError> {
    let file = File::open(input_zip)?;
    let mut archive = ZipArchive::new(file)?;
    fs::create_dir_all(output_dir)?;
    let filter = PathFilter::new(include, exclude)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let raw_name = entry.name_raw();
        let decoded_name = encoding.decode(raw_name)?;
        if !filter.allows(&decoded_name) {
            continue;
        }
        let safe_rel = sanitize_relative_path(&decoded_name)?;
        let outpath = output_dir.join(&safe_rel);

        if entry.is_dir() {
            fs::create_dir_all(&outpath)?;
            continue;
        }

        if let Some(parent) = outpath.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut outfile = File::create(&outpath)?;
        io::copy(&mut entry, &mut outfile)?;
        outfile.flush()?;
    }

    Ok(())
}

fn sanitize_relative_path(path: &str) -> Result<PathBuf, RzipError> {
    let candidate = Path::new(path);
    let mut safe = PathBuf::new();
    for comp in candidate.components() {
        match comp {
            Component::Normal(part) => safe.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(RzipError::UnsafePath(path.to_string()));
            }
        }
    }
    if safe.as_os_str().is_empty() {
        return Err(RzipError::UnsafePath(path.to_string()));
    }
    Ok(safe)
}

#[cfg(test)]
mod tests {
    use super::sanitize_relative_path;

    #[test]
    fn rejects_parent_dir() {
        assert!(sanitize_relative_path("../evil.txt").is_err());
    }
}
