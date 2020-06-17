#[cfg(feature = "gzip")]
pub use self::gzip::Gzip;

#[cfg(feature = "gzip")]
mod gzip {
    use std::fs::{create_dir_all, File};
    use std::io::copy;
    use std::path::Path;

    use flate2::read::GzDecoder;

    use crate::{Compressed, Error};

    pub struct Gzip {
        archive: GzDecoder<File>,
    }

    impl Gzip {
        pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
            let archive = File::open(&path)?;
            let archive = GzDecoder::new(archive);

            Ok(Self { archive })
        }
    }

    impl Compressed for Gzip {
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
