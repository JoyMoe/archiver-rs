pub mod bzip2;
pub mod gzip;
pub mod tar;
pub mod xz;
pub mod zip;

type Error = Box<dyn std::error::Error>;
