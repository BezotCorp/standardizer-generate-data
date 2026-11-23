mod arguments;
mod dataset;
mod generator;

use arguments::Arguments;
use generator::Generator;
use std::process::ExitCode;

fn main() -> ExitCode {
    let result = Arguments::parse().and_then(|arguments| match arguments {
        Arguments::Check { ron_directory } => Generator::check(&ron_directory),
        Arguments::Generate {
            ron_directory,
            bin_directory,
        } => Generator::generate(&ron_directory, &bin_directory),
    });

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("standardizer-generate-data: {error}");
            ExitCode::FAILURE
        }
    }
}
