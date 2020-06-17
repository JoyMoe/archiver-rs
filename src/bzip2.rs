#[cfg(feature = "bzip2")]
pub use self::bzip2::Bzip2;

#[cfg(feature = "bzip2")]
mod bzip2 {
    use std::fs::{create_dir_all, File};
    use std::io::copy;
    use std::path::Path;

    use bzip2::read::BzDecoder;

    use crate::{Compressed, Error};

    pub struct Bzip2 {
        archive: BzDecoder<File>,
    }

    impl Bzip2 {
        pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
            let archive = File::open(&path)?;
            let archive = BzDecoder::new(archive);

            Ok(Self { archive })
        }
    }

    impl Compressed for Bzip2 {
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
