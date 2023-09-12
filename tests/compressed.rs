use std::path::Path;

use archiver_rs::{Bzip2, Compressed, Gzip, Xz};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn test_decompress(mut compressed: impl Compressed, expected: &str) -> Result<()> {
    let sandbox = tempfile::tempdir()?;
    let decompressed_file_path = sandbox.path().join("sample.txt");
    compressed.decompress(Path::new(&decompressed_file_path))?;
    assert_eq!(std::fs::read_to_string(decompressed_file_path)?, expected);
    Ok(())
}

#[test]
fn test_gzipped_decompress_path() -> Result<()> {
    test_decompress(
        Gzip::open(Path::new("tests/sample/sample.txt.gz"))?,
        "sample\n",
    )
}

#[test]
fn test_gzipped_decompress_str() -> Result<()> {
    test_decompress(Gzip::open("tests/sample/sample.txt.gz")?, "sample\n")
}

#[test]
fn test_bzipped_decompress_path() -> Result<()> {
    test_decompress(
        Bzip2::open(Path::new("tests/sample/sample.txt.bz2"))?,
        "sample\n",
    )
}

#[test]
fn test_bzipped_decompress_str() -> Result<()> {
    test_decompress(Bzip2::open("tests/sample/sample.txt.bz2")?, "sample\n")
}

#[test]
fn test_xzipped_decompress_path() -> Result<()> {
    test_decompress(
        Xz::open(Path::new("tests/sample/sample.txt.xz"))?,
        "sample\n",
    )
}

#[test]
fn test_xzipped_decompress_str() -> Result<()> {
    test_decompress(Xz::open("tests/sample/sample.txt.xz")?, "sample\n")
}
