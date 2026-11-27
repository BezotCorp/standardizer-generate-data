use crate::generator::Generator;
use standardizer_common::{routes::Routing, tools::ToolConfiguration};
use std::{error::Error, fs, path::Path};
use tempfile::tempdir;

const ROUTING: &str = r#"(
    routes: [
        (extensions: ["rs"], tools: [Rustfmt]),
    ],
)"#;

const TOOLS: &str = r#"(
    tools: [
        (
            id: Biome,
            minimum_version: "2.6.0",
            managed_version: "2.6.8",
        ),
    ],
)"#;

fn write_sources(directory: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(directory)?;
    fs::write(directory.join("routing.ron"), ROUTING)?;
    fs::write(directory.join("tools.ron"), TOOLS)?;
    Ok(())
}

#[test]
fn check_valid_sources() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;

    Generator::check(directory.path())?;

    assert_eq!(fs::read_dir(directory.path())?.count(), 2);
    assert!(!directory.path().join("routing.bin").exists());
    assert!(!directory.path().join("tools.bin").exists());

    Ok(())
}

#[test]
fn check_missing_routing() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;
    fs::remove_file(directory.path().join("routing.ron"))?;

    let result = Generator::check(directory.path());

    assert!(result.is_err());

    Ok(())
}

#[test]
fn check_missing_tools() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;
    fs::remove_file(directory.path().join("tools.ron"))?;

    let result = Generator::check(directory.path());

    assert!(result.is_err());

    Ok(())
}

#[test]
fn check_invalid_routing() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;
    fs::write(directory.path().join("routing.ron"), "invalid")?;

    assert!(Generator::check(directory.path()).is_err());

    Ok(())
}

#[test]
fn check_invalid_tools() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;
    fs::write(directory.path().join("tools.ron"), "invalid")?;

    assert!(Generator::check(directory.path()).is_err());

    Ok(())
}

#[test]
fn generate_creates_output_directory() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let source = directory.path().join("source");
    let output = directory.path().join("nested").join("output");

    write_sources(&source)?;

    assert!(!output.exists());

    Generator::generate(&source, &output)?;

    assert!(output.is_dir());
    assert!(output.join("routing.bin").is_file());
    assert!(output.join("tools.bin").is_file());
    assert_eq!(fs::read_dir(&output)?.count(), 2);
    assert_eq!(fs::read_dir(&source)?.count(), 2);

    Ok(())
}

#[test]
fn generate_produces_expected_binary_data() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let source = directory.path().join("source");
    let output = directory.path().join("output");

    write_sources(&source)?;

    Generator::generate(&source, &output)?;

    let routing_binary = fs::read(output.join("routing.bin"))?;
    let tools_binary = fs::read(output.join("tools.bin"))?;

    let expected_routing = Routing::from_ron(ROUTING)?.encode()?;
    let expected_tools = ToolConfiguration::from_ron(TOOLS)?.encode()?;

    assert_eq!(routing_binary, expected_routing);
    assert_eq!(tools_binary, expected_tools);

    Routing::decode(&routing_binary)?;
    ToolConfiguration::decode(&tools_binary)?;

    Ok(())
}

#[test]
fn generate_in_source_directory() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;

    Generator::generate(directory.path(), directory.path())?;

    assert_eq!(
        fs::read_to_string(directory.path().join("routing.ron"))?,
        ROUTING
    );

    assert_eq!(
        fs::read_to_string(directory.path().join("tools.ron"))?,
        TOOLS
    );

    assert!(directory.path().join("routing.bin").is_file());
    assert!(directory.path().join("tools.bin").is_file());
    assert_eq!(fs::read_dir(directory.path())?.count(), 4);

    Ok(())
}

#[test]
fn generate_replaces_existing_binary_files() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;

    let routing_path = directory.path().join("routing.bin");
    let tools_path = directory.path().join("tools.bin");

    fs::write(&routing_path, [255; 1024])?;
    fs::write(&tools_path, [255; 1024])?;

    Generator::generate(directory.path(), directory.path())?;

    assert_eq!(
        fs::read(&routing_path)?,
        Routing::from_ron(ROUTING)?.encode()?
    );

    assert_eq!(
        fs::read(&tools_path)?,
        ToolConfiguration::from_ron(TOOLS)?.encode()?
    );

    Ok(())
}

#[test]
fn generate_invalid_routing_does_not_create_output() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let source = directory.path().join("source");
    let output = directory.path().join("output");

    write_sources(&source)?;
    fs::write(source.join("routing.ron"), "invalid")?;

    assert!(Generator::generate(&source, &output).is_err());
    assert!(!output.exists());

    Ok(())
}

#[test]
fn generate_invalid_tools_does_not_create_output() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let source = directory.path().join("source");
    let output = directory.path().join("output");

    write_sources(&source)?;
    fs::write(source.join("tools.ron"), "invalid")?;

    assert!(Generator::generate(&source, &output).is_err());
    assert!(!output.exists());

    Ok(())
}

#[test]
fn generate_invalid_sources_preserves_existing_binary_files() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;

    Generator::generate(directory.path(), directory.path())?;

    let routing_path = directory.path().join("routing.bin");
    let tools_path = directory.path().join("tools.bin");

    let routing_before = fs::read(&routing_path)?;
    let tools_before = fs::read(&tools_path)?;

    fs::write(directory.path().join("tools.ron"), "invalid")?;

    assert!(Generator::generate(directory.path(), directory.path()).is_err());

    assert_eq!(fs::read(&routing_path)?, routing_before);
    assert_eq!(fs::read(&tools_path)?, tools_before);

    Ok(())
}

#[test]
fn generate_rejects_file_as_output_directory() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let source = directory.path().join("source");
    let output = directory.path().join("output");

    write_sources(&source)?;
    fs::write(&output, "existing file")?;

    assert!(Generator::generate(&source, &output).is_err());
    assert_eq!(fs::read_to_string(&output)?, "existing file");

    Ok(())
}
