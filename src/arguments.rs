use std::{
    env,
    error::Error,
    io,
    path::PathBuf,
};

const MISSING_COMMAND: &str = "missing command: expected check or generate";
const UNKNOWN_COMMAND: &str = "unknown command: expected check or generate";
const MISSING_RON_DIRECTORY: &str = "missing RON directory";
const MISSING_BIN_DIRECTORY: &str = "missing BIN directory";
const EXPECTED_CHECK_ARGUMENTS: &str =
    "check expects exactly one RON directory";
const EXPECTED_GENERATE_ARGUMENTS: &str =
    "generate expects exactly one RON directory and one BIN directory";

pub(crate) enum Arguments {
    Check {
        ron_directory: PathBuf,
    },
    Generate {
        ron_directory: PathBuf,
        bin_directory: PathBuf,
    },
}

impl Arguments {
    pub(crate) fn parse() -> Result<Self, Box<dyn Error>> {
        let mut arguments = env::args_os();
        arguments.next();

        let command = arguments.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                MISSING_COMMAND,
            )
        })?;

        match command.to_str() {
            Some("check") => {
                let ron_directory = arguments
                    .next()
                    .map(PathBuf::from)
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            MISSING_RON_DIRECTORY,
                        )
                    })?;

                if arguments.next().is_some() {
                    return Err(
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            EXPECTED_CHECK_ARGUMENTS,
                        )
                        .into(),
                    );
                }

                Ok(Self::Check { ron_directory })
            }
            Some("generate") => {
                let ron_directory = arguments
                    .next()
                    .map(PathBuf::from)
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            MISSING_RON_DIRECTORY,
                        )
                    })?;

                let bin_directory = arguments
                    .next()
                    .map(PathBuf::from)
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            MISSING_BIN_DIRECTORY,
                        )
                    })?;

                if arguments.next().is_some() {
                    return Err(
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            EXPECTED_GENERATE_ARGUMENTS,
                        )
                        .into(),
                    );
                }

                Ok(Self::Generate {
                    ron_directory,
                    bin_directory,
                })
            }
            _ => Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    UNKNOWN_COMMAND,
                )
                .into(),
            ),
        }
    }
}
