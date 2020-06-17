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

trait Archive {
    fn contains(&mut self, file: String) -> Result<bool, Error>;

    fn extract(&mut self, destination: &Path) -> Result<(), Error>;

    fn extract_single(&mut self, target: &Path, file: String) -> Result<(), Error>;

    fn files(&mut self) -> Result<Vec<String>, Error>;
}

trait Compressed {
    fn decompress(&mut self, target: &Path) -> Result<(), Error>;
}
