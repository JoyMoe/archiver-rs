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
    fn contains<S: Into<String>>(&mut self, file: S) -> Result<bool, Error>;

    fn extract<T: AsRef<Path>>(&mut self, destination: T) -> Result<(), Error>;

    fn extract_single<T: AsRef<Path>, S: Into<String>>(
        &mut self,
        target: T,
        file: S,
    ) -> Result<(), Error>;

    fn files(&mut self) -> Result<Vec<String>, Error>;
}

trait Compressed {
    fn decompress<T: AsRef<Path>>(&mut self, target: T) -> Result<(), Error>;
}
