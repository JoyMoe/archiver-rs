#[cfg(feature = "zip")]
mod zip {
    use std::fs::{create_dir_all, File};
    use std::io::copy;
    use std::path::Path;

    use zip::ZipArchive;

    use crate::Error;

    fn contains_file(archive: &mut ZipArchive<File>, file: String) -> Result<bool, Error> {
        match archive.by_name(&file) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn contains<P: AsRef<Path>, S: Into<String>>(archive: P, file: S) -> Result<bool, Error> {
        let archive = archive.as_ref();
        let file = file.into();

        let archive = File::open(&archive)?;
        let mut archive = ZipArchive::new(archive)?;

        contains_file(&mut archive, file)
    }

    pub fn extract<P: AsRef<Path>, T: AsRef<Path>>(
        archive: P,
        destination: T,
    ) -> Result<(), Error> {
        let archive = archive.as_ref();
        let destination = destination.as_ref();

        let archive = File::open(&archive)?;
        let mut archive = ZipArchive::new(archive)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let target = file.sanitized_name();
            let target = destination.join(target);

            if (&*file.name()).ends_with('/') {
                create_dir_all(&target)?;
            } else {
                if let Some(p) = target.parent() {
                    if !p.exists() {
                        create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&target)?;
                copy(&mut file, &mut outfile)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::fs::{set_permissions, Permissions};
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    set_permissions(&target, Permissions::from_mode(mode))?;
                }
            }
        }

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
        let mut archive = ZipArchive::new(archive)?;

        let mut f = archive.by_name(&file).map_err(|_| "NOT FOUND")?;

        let mut target = File::create(&target)?;
        copy(&mut f, &mut target)?;

        return Ok(());
    }
}
