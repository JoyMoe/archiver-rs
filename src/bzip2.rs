use bzip2::read::BzDecoder;
use std::{fs::File, io::Read, path::Path};

use crate::Compressed;

pub struct Bzip2<R: Read> {
    archive: BzDecoder<R>,
}

impl Bzip2<File> {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let archive = File::open(path)?;

        Self::new(archive)
    }
}

impl<R: Read> Bzip2<R> {
    pub fn new(r: R) -> std::io::Result<Self> {
        let archive = BzDecoder::new(r);

        Ok(Self { archive })
    }
}

impl<R: Read> Compressed for Bzip2<R> {}

impl<R: Read> Read for Bzip2<R> {
    fn read(&mut self, into: &mut [u8]) -> std::io::Result<usize> {
        self.archive.read(into)
    }
}
