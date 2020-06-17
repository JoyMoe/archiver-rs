#[cfg(feature = "zip")]
pub use self::zip::Zip;

#[cfg(feature = "zip")]
mod zip {
    use std::fs::{create_dir_all, File};
    use std::io::copy;
    use std::path::Path;

    use zip::ZipArchive;

    use crate::{Archive, Error};

    pub struct Zip {
        archive: ZipArchive<File>,
    }

    impl Zip {
        pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
            let archive = File::open(&path)?;
            let archive = ZipArchive::new(archive)?;

            Ok(Self { archive })
        }
    }

    impl Archive for Zip {
        fn contains<S: Into<String>>(&mut self, file: S) -> Result<bool, Error> {
            let file = file.into();

            let result = match self.archive.by_name(&file) {
                Ok(_) => true,
                Err(_) => false,
            };

            Ok(result)
        }

        fn extract<T: AsRef<Path>>(&mut self, destination: T) -> Result<(), Error> {
            let destination = destination.as_ref();

            for i in 0..self.archive.len() {
                let mut file = self.archive.by_index(i)?;
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

        fn extract_single<T: AsRef<Path>, S: Into<String>>(
            &mut self,
            target: T,
            file: S,
        ) -> Result<(), Error> {
            let target = target.as_ref();
            let file = file.into();

            if let Some(p) = target.parent() {
                if !p.exists() {
                    create_dir_all(&p)?;
                }
            }

            let mut f = self.archive.by_name(&file).map_err(|_| "NOT FOUND")?;

            let mut target = File::create(&target)?;
            copy(&mut f, &mut target)?;

            return Ok(());
        }

        fn files(&mut self) -> Result<Vec<String>, Error> {
            let files = self.archive.file_names().map(|e| e.into()).collect();

            Ok(files)
        }
    }
}
