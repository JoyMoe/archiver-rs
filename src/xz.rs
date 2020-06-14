#[cfg(feature = "xz")]
mod xz {
    use std::fs::{create_dir_all, File};
    use std::io::BufReader;
    use std::path::Path;

    use lzma_rs::error;
    use lzma_rs::xz_decompress;

    use crate::Error;

    pub fn decode<P: AsRef<Path>, T: AsRef<Path>>(archive: P, target: T) -> Result<(), Error> {
        let archive = archive.as_ref();
        let target = target.as_ref();

        if let Some(p) = target.parent() {
            if !p.exists() {
                create_dir_all(&p)?;
            }
        }

        let archive = File::open(&archive)?;
        let mut archive = BufReader::new(archive);

        let mut output = File::create(&target)?;
        match xz_decompress(&mut archive, &mut output) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                error::Error::IOError(err) => Err(err)?,
                error::Error::LZMAError(err) => Err(err)?,
                error::Error::XZError(err) => Err(err)?,
            },
        }
    }
}
