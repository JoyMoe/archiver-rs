#[cfg(feature = "gzip")]
pub use self::gzip::Gzip;

#[cfg(feature = "gzip")]
mod gzip {
    use std::fs::{create_dir_all, File};
    use std::io::{copy, Read};
    use std::path::Path;

    use flate2::read::GzDecoder;

    use crate::{Compressed, Error};

    pub struct Gzip<R: Read> {
        archive: GzDecoder<R>,
    }

    impl Gzip<File> {
        pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
            let archive = File::open(&path)?;

            Self::new(archive)
        }
    }

    impl<R: Read> Gzip<R> {
        pub fn new(r: R) -> std::io::Result<Self> {
            let archive = GzDecoder::new(r);

            Ok(Self { archive })
        }
    }

    impl<R: Read> Compressed for Gzip<R> {
        fn decompress<T: AsRef<Path>>(&mut self, target: T) -> Result<(), Error> {
            let target = target.as_ref();

            if let Some(p) = target.parent() {
                if !p.exists() {
                    create_dir_all(&p)?;
                }
            }

            let mut output = File::create(&target)?;
            copy(&mut self.archive, &mut output)?;

            Ok(())
        }
    }
}
