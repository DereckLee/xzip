use std::fs;
use std::io::Write;
use std::process::Command;

use tempfile::tempdir;
use xzip::codec::EncodingKind;
use xzip::pack::pack_path;
use xzip::unpack::unpack_archive;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

#[test]
fn round_trip_pack_unpack_utf8() {
    let temp = tempdir().expect("temp dir");
    let input_dir = temp.path().join("input");
    let output_dir = temp.path().join("output");
    fs::create_dir_all(input_dir.join("sub")).expect("create input subdir");
    fs::write(input_dir.join("sub/hello.txt"), b"hello").expect("write file");
    fs::write(input_dir.join("root.txt"), b"root").expect("write file");

    let archive = temp.path().join("archive.zip");
    pack_path(&input_dir, &archive, EncodingKind::Utf8, true, &[], &[]).expect("pack");
    unpack_archive(&archive, &output_dir, EncodingKind::Utf8, &[], &[]).expect("unpack");

    let unpacked = fs::read(output_dir.join("sub/hello.txt")).expect("read unpacked file");
    assert_eq!(unpacked, b"hello");
}

#[test]
fn invalid_encoding_label_rejected() {
    assert!(EncodingKind::from_label("bad-encoding").is_err());
}

#[test]
fn blocks_zip_slip_paths() {
    let temp = tempdir().expect("temp dir");
    let archive_path = temp.path().join("unsafe.zip");
    {
        let file = fs::File::create(&archive_path).expect("create archive");
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        zip.start_file("../outside.txt", options)
            .expect("start file");
        zip.write_all(b"attack").expect("write attack payload");
        zip.finish().expect("finalize archive");
    }

    let output = temp.path().join("extract");
    let result = unpack_archive(&archive_path, &output, EncodingKind::Utf8, &[], &[]);
    assert!(result.is_err());
}

#[test]
fn non_recursive_pack_skips_nested_files() {
    let temp = tempdir().expect("temp dir");
    let input_dir = temp.path().join("input");
    fs::create_dir_all(input_dir.join("nested")).expect("create nested dir");
    fs::write(input_dir.join("root.txt"), b"root").expect("write root");
    fs::write(input_dir.join("nested/deep.txt"), b"deep").expect("write nested");

    let archive = temp.path().join("archive.zip");
    pack_path(&input_dir, &archive, EncodingKind::Utf8, false, &[], &[]).expect("pack");

    let output_dir = temp.path().join("output");
    unpack_archive(&archive, &output_dir, EncodingKind::Utf8, &[], &[]).expect("unpack");
    assert!(output_dir.join("root.txt").exists());
    assert!(!output_dir.join("nested/deep.txt").exists());
}

#[test]
fn cli_defaults_to_utf8_when_encoding_omitted() {
    let temp = tempdir().expect("temp dir");
    let input_dir = temp.path().join("input");
    fs::create_dir_all(&input_dir).expect("create input dir");
    fs::write(input_dir.join("hello.txt"), b"hello").expect("write input");

    let archive = temp.path().join("archive.zip");
    let output_dir = temp.path().join("output");
    let bin = env!("CARGO_BIN_EXE_xzip");

    let pack_status = Command::new(bin)
        .args([
            "pack",
            "-i",
            input_dir.to_string_lossy().as_ref(),
            "-o",
            archive.to_string_lossy().as_ref(),
            "-r",
        ])
        .status()
        .expect("run pack");
    assert!(pack_status.success());

    let unpack_status = Command::new(bin)
        .args([
            "unpack",
            "-i",
            archive.to_string_lossy().as_ref(),
            "-o",
            output_dir.to_string_lossy().as_ref(),
        ])
        .status()
        .expect("run unpack");
    assert!(unpack_status.success());
    assert_eq!(
        fs::read(output_dir.join("hello.txt")).expect("read output"),
        b"hello"
    );
}
