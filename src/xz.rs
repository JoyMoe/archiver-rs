use std::{fs::File, io::Read, path::Path};
use xz2::read::XzDecoder;

use crate::Compressed;

pub struct Xz<R: Read> {
    archive: XzDecoder<R>,
}

impl Xz<File> {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let archive = File::open(path)?;

        Self::new(archive)
    }
}

impl<R: Read> Xz<R> {
    pub fn new(r: R) -> std::io::Result<Self> {
        let archive = XzDecoder::new(r);

        Ok(Self { archive })
    }
}

impl<R: Read> Compressed for Xz<R> {}

impl<R: Read> Read for Xz<R> {
    fn read(&mut self, into: &mut [u8]) -> std::io::Result<usize> {
        self.archive.read(into)
    }
}
