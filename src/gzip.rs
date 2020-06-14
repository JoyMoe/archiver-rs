#[cfg(feature = "gzip")]
mod gzip {
    use std::fs::{create_dir_all, File};
    use std::io::copy;
    use std::path::Path;

    use flate2::read::GzDecoder;

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
        let mut archive = GzDecoder::new(archive);

        let mut output = File::create(&target)?;
        copy(&mut archive, &mut output)?;

        Ok(())
    }
}
