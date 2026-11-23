mod arguments;
mod dataset;

use arguments::Arguments;
use dataset::Dataset;
use standardizer_common::{routes::Routing, tools::ToolConfiguration};
use std::{error::Error, fs, process::ExitCode};

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse()?;

    let routing_path = arguments.ron_directory().join("routing.ron");
    let tools_path = arguments.ron_directory().join("tools.ron");

    let routing: Routing = Dataset::read(&routing_path)?;
    routing
        .validate()
        .map_err(|error| format!("{}: {error}", routing_path.display()))?;

    let tools: ToolConfiguration = Dataset::read(&tools_path)?;
    tools
        .validate()
        .map_err(|error| format!("{}: {error}", tools_path.display()))?;

    let routing_binary = Dataset::encode(&routing)?;
    let tools_binary = Dataset::encode(&tools)?;

    fs::create_dir_all(arguments.bin_directory())?;

    fs::write(
        arguments.bin_directory().join("routing.bin"),
        routing_binary,
    )?;

    fs::write(arguments.bin_directory().join("tools.bin"), tools_binary)?;

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("standardizer-generate-data: {error}");
            ExitCode::FAILURE
        }
    }
}
