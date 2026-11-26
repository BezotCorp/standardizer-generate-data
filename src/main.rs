mod arguments;
mod dataset;
mod generation_result;
mod generator;

use arguments::Arguments;
use generator::Generator;
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let result = Arguments::parse().and_then(|arguments| match arguments {
        Arguments::Check { ron_directory } => Generator::check(&ron_directory),
        Arguments::Generate {
            ron_directory,
            bin_directory,
        } => {
            let result = Generator::generate(&ron_directory, &bin_directory)?;
            let response = ron::to_string(&result)?;
            writeln!(io::stdout().lock(), "{response}")?;
            Ok(())
        }
    });

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("standardizer-generate-data: {error}");
            ExitCode::FAILURE
        }
    }
}
