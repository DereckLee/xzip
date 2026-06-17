use std::fs;
use std::io::Write;

use rzip::codec::EncodingKind;
use rzip::pack::pack_path;
use rzip::unpack::unpack_archive;
use tempfile::tempdir;
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
    pack_path(&input_dir, &archive, EncodingKind::Utf8).expect("pack");
    unpack_archive(&archive, &output_dir, EncodingKind::Utf8).expect("unpack");

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
    let result = unpack_archive(&archive_path, &output, EncodingKind::Utf8);
    assert!(result.is_err());
}
