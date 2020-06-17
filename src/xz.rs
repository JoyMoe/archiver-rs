#[cfg(feature = "xz")]
pub use self::xz::Xz;

#[cfg(feature = "xz")]
mod xz {
    use std::fs::{create_dir_all, File};
    use std::io::{BufReader, Read};
    use std::path::Path;

    use lzma_rs::error;
    use lzma_rs::xz_decompress;

    use crate::{Compressed, Error};

    pub struct Xz<R: Read> {
        archive: BufReader<R>,
    }

    impl Xz<File> {
        pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
            let archive = File::open(&path)?;

            Self::new(archive)
        }
    }

    impl<R: Read> Xz<R> {
        pub fn new(r: R) -> std::io::Result<Self> {
            let archive = BufReader::new(r);

            Ok(Self { archive })
        }
    }

    impl<R: Read> Compressed for Xz<R> {
        fn decompress<T: AsRef<Path>>(&mut self, target: T) -> Result<(), Error> {
            let target = target.as_ref();

            if let Some(p) = target.parent() {
                if !p.exists() {
                    create_dir_all(&p)?;
                }
            }

            let mut output = File::create(&target)?;
            match xz_decompress(&mut self.archive, &mut output) {
                Ok(_) => Ok(()),
                Err(err) => match err {
                    error::Error::IOError(err) => Err(err)?,
                    error::Error::LZMAError(err) => Err(err)?,
                    error::Error::XZError(err) => Err(err)?,
                },
            }
        }
    }
}
