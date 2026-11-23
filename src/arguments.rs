use std::{env, error::Error, io, path::PathBuf};

const MISSING_RON_DIRECTORY: &str = "missing RON directory";
const MISSING_BIN_DIRECTORY: &str = "missing BIN directory";
const EXPECTED_TWO_DIRECTORIES: &str = "expected exactly two directories";

pub(crate) struct Arguments {
    ron_directory: PathBuf,
    bin_directory: PathBuf,
}

impl Arguments {
    pub(crate) fn ron_directory(&self) -> &PathBuf {
        &self.ron_directory
    }

    pub(crate) fn bin_directory(&self) -> &PathBuf {
        &self.bin_directory
    }

    pub(crate) fn parse() -> Result<Self, Box<dyn Error>> {
        let mut arguments = env::args_os();
        arguments.next();

        let ron_directory = arguments
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, MISSING_RON_DIRECTORY))?;

        let bin_directory = arguments
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, MISSING_BIN_DIRECTORY))?;

        if arguments.next().is_some() {
            return Err(
                io::Error::new(io::ErrorKind::InvalidInput, EXPECTED_TWO_DIRECTORIES).into(),
            );
        }

        Ok(Self {
            ron_directory,
            bin_directory,
        })
    }
}
