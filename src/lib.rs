use std::path::Path;

pub use crate::bzip2::Bzip2;
pub use crate::gzip::Gzip;
pub use crate::tar::Tar;
pub use crate::xz::Xz;
pub use crate::zip::Zip;

mod bzip2;
mod gzip;
mod tar;
mod xz;
mod zip;

type Error = Box<dyn std::error::Error>;

pub trait Archive {
    fn contains(&mut self, file: String) -> Result<bool, Error>;

    fn extract(&mut self, destination: &Path) -> Result<(), Error>;

    fn extract_single(&mut self, target: &Path, file: String) -> Result<(), Error>;

    fn files(&mut self) -> Result<Vec<String>, Error>;
}

pub trait Compressed: std::io::Read {
    fn decompress(&mut self, target: &Path) -> Result<(), Error>;
}

pub fn open(path: &Path) -> std::io::Result<Box<dyn Archive>> {
    use std::fs::File;
    use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

    let mut file = File::open(&path)?;
    let mut buffer = [0u8; 2];
    file.read(&mut buffer)?;
    file.seek(SeekFrom::Start(0))?;

    match buffer {
        #[cfg(all(feature = "bzip2", feature = "tar"))]
        [0x42, 0x5A] => Ok(Box::new(Tar::new(Bzip2::new(file)?)?)), // .tar.gz
        #[cfg(all(feature = "gzip", feature = "tar"))]
        [0x1F, 0x8B] => Ok(Box::new(Tar::new(Gzip::new(file)?)?)), // .tar.gz
        #[cfg(feature = "zip")]
        [0x50, 0x4B] => Ok(Box::new(Zip::new(file)?)), // .zip
        _ => Err(Error::from(ErrorKind::InvalidData))?,
    }
}
