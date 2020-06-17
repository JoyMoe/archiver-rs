#[cfg(feature = "tar")]
pub use self::tar::Tar;

#[cfg(feature = "tar")]
mod tar {
    use std::fs::{create_dir_all, File};
    use std::io::{copy, Read};
    use std::path::Path;

    use crate::{Archive, Error};

    pub struct Tar<R: Read> {
        archive: tar::Archive<R>,
    }

    impl Tar<File> {
        pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
            let archive = File::open(&path)?;

            Self::new(archive)
        }
    }

    impl<R: Read> Tar<R> {
        pub fn new(r: R) -> std::io::Result<Self> {
            let archive = tar::Archive::new(r);

            Ok(Self { archive })
        }
    }

    impl<R: Read> Archive for Tar<R> {
        fn contains<S: Into<String>>(&mut self, file: S) -> Result<bool, Error> {
            let file = file.into();

            for f in self.archive.entries()? {
                let f = f?;
                let name = f.path()?;

                if name.to_str().ok_or_else(|| "NO NAME")? == file {
                    return Ok(true);
                }
            }

            Ok(false)
        }

        fn extract<T: AsRef<Path>>(&mut self, destination: T) -> Result<(), Error> {
            let destination = destination.as_ref();

            if !destination.exists() {
                create_dir_all(&destination)?;
            }

            self.archive.unpack(destination)?;

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

            for f in self.archive.entries()? {
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

        fn files(&mut self) -> Result<Vec<String>, Error> {
            let files = self
                .archive
                .entries()?
                .into_iter()
                .map(|e| e.unwrap().path().unwrap().to_str().unwrap().into())
                .collect();

            Ok(files)
        }
    }
}
