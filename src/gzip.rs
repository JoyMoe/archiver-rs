use flate2::read::GzDecoder;
use std::{fs::File, io::Read, path::Path};

use crate::Compressed;

pub struct Gzip<R: Read> {
    archive: GzDecoder<R>,
}

impl Gzip<File> {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let archive = File::open(path)?;

        Self::new(archive)
    }
}

impl<R: Read> Gzip<R> {
    pub fn new(r: R) -> std::io::Result<Self> {
        let archive = GzDecoder::new(r);

        Ok(Self { archive })
    }
}

impl<R: Read> Compressed for Gzip<R> {}

impl<R: Read> Read for Gzip<R> {
    fn read(&mut self, into: &mut [u8]) -> std::io::Result<usize> {
        self.archive.read(into)
    }
}
