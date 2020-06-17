#[cfg(feature = "bzip2")]
pub use self::bzip2::Bzip2;

#[cfg(feature = "bzip2")]
mod bzip2 {
    use std::fs::{create_dir_all, File};
    use std::io::{copy, Read};
    use std::path::Path;

    use bzip2::read::BzDecoder;

    use crate::{Compressed, Error};

    pub struct Bzip2<R: Read> {
        archive: BzDecoder<R>,
    }

    impl Bzip2<File> {
        pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
            let archive = File::open(&path)?;

            Self::new(archive)
        }
    }

    impl<R: Read> Bzip2<R> {
        pub fn new(r: R) -> std::io::Result<Self> {
            let archive = BzDecoder::new(r);

            Ok(Self { archive })
        }
    }

    impl<R: Read> Compressed for Bzip2<R> {
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
