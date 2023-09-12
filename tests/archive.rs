use archiver_rs::{Archive, Bzip2, Gzip, Tar, Xz};
use std::{fs::read_to_string, path::Path};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

mod test_files {
    use super::*;

    fn test(mut archive: impl Archive) -> Result<()> {
        let actual = archive.files()?;
        let expected: Vec<String> = vec!["sample/".into(), "sample/sample.txt".into()];
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn tar_archive() -> Result<()> {
        test(Tar::open(Path::new("tests/sample/sample.tar"))?)
    }

    #[test]
    fn tar_gzipped_archive() -> Result<()> {
        test(Tar::new(Gzip::open(Path::new(
            "tests/sample/sample.tar.gz",
        ))?)?)
    }

    #[test]
    fn tar_bzipped_archive() -> Result<()> {
        test(Tar::new(Bzip2::open(Path::new(
            "tests/sample/sample.tar.bz2",
        ))?)?)
    }

    #[test]
    fn tar_xzipped_archive() -> Result<()> {
        test(Tar::new(Xz::open(Path::new(
            "tests/sample/sample.tar.xz",
        ))?)?)
    }
}

mod test_extract {

    use super::*;

    fn test(mut archive: impl Archive) -> Result<()> {
        let sandbox = tempfile::tempdir()?;
        archive.extract(sandbox.path())?;

        assert_eq!(
            read_to_string(sandbox.path().join("sample/sample.txt"))?,
            "sample\n"
        );

        Ok(())
    }

    #[test]
    fn tar_archive() -> Result<()> {
        test(Tar::open(Path::new("tests/sample/sample.tar"))?)
    }

    #[test]
    fn tar_gzipped_archive() -> Result<()> {
        test(Tar::new(Gzip::open(Path::new(
            "tests/sample/sample.tar.gz",
        ))?)?)
    }

    #[test]
    fn tar_bzipped_archive() -> Result<()> {
        test(Tar::new(Bzip2::open(Path::new(
            "tests/sample/sample.tar.bz2",
        ))?)?)
    }

    #[test]
    fn tar_xzipped_archive() -> Result<()> {
        test(Tar::new(Xz::open(Path::new(
            "tests/sample/sample.tar.xz",
        ))?)?)
    }
}

mod test_extract_single {
    use super::*;

    fn test(mut archive: impl Archive) -> Result<()> {
        let sandbox = tempfile::tempdir()?;
        let decompressed_path = sandbox.path().join("sample/sample.txt");

        archive.extract_single(&decompressed_path, "sample/sample.txt".into())?;
        assert_eq!(read_to_string(&decompressed_path)?, "sample\n");

        Ok(())
    }

    #[test]
    fn tar_archive() -> Result<()> {
        test(Tar::open(Path::new("tests/sample/sample.tar"))?)
    }

    #[test]
    fn tar_gzipped_archive() -> Result<()> {
        test(Tar::new(Gzip::open(Path::new(
            "tests/sample/sample.tar.gz",
        ))?)?)
    }

    #[test]
    fn tar_bzipped_archive() -> Result<()> {
        test(Tar::new(Bzip2::open(Path::new(
            "tests/sample/sample.tar.bz2",
        ))?)?)
    }

    #[test]
    fn tar_xzipped_archive() -> Result<()> {
        test(Tar::new(Xz::open(Path::new(
            "tests/sample/sample.tar.xz",
        ))?)?)
    }
}
