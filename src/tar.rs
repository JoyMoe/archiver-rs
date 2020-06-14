#[cfg(feature = "tar")]
mod tar {
    use std::fs::{create_dir_all, File};
    use std::io::copy;
    use std::path::Path;

    use tar::Archive;

    use crate::Error;

    fn contains_file(archive: &mut Archive<File>, file: String) -> Result<bool, Error> {
        for f in archive.entries()? {
            let f = f?;
            let name = f.path()?;

            if name.to_str().ok_or_else(|| "NO NAME")? == file {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn contains<P: AsRef<Path>, S: Into<String>>(archive: P, file: S) -> Result<bool, Error> {
        let archive = archive.as_ref();
        let file = file.into();

        let archive = File::open(&archive)?;
        let mut archive = Archive::new(archive);

        contains_file(&mut archive, file)
    }

    pub fn extract<P: AsRef<Path>, T: AsRef<Path>>(
        archive: P,
        destination: T,
    ) -> Result<(), Error> {
        let archive = archive.as_ref();
        let destination = destination.as_ref();

        if !destination.exists() {
            create_dir_all(&destination)?;
        }

        let archive = File::open(&archive)?;
        let mut archive = Archive::new(archive);

        archive.unpack(destination)?;

        Ok(())
    }

    pub fn extract_single<P: AsRef<Path>, T: AsRef<Path>, S: Into<String>>(
        archive: P,
        destination: T,
        file: S,
    ) -> Result<(), Error> {
        let archive = archive.as_ref();
        let destination = destination.as_ref();
        let file = file.into();

        if !destination.exists() {
            create_dir_all(&destination)?;
        }

        let target = destination.join(&file);

        let archive = File::open(&archive)?;
        let mut archive = Archive::new(archive);

        for f in archive.entries()? {
            let mut f = f?;
            let name = f.path()?;

            if name.to_str().ok_or_else(|| "NO NAME")? == &file {
                let mut target = File::create(&target)?;
                copy(&mut f, &mut target)?;

                return Ok(());
            }
        }

        Err("NOT FOUND")?
    }
}
