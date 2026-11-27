use semver::Version;
use standardizer_common::{introspection::Package, routes::Routing, tools::ToolConfiguration};
use std::{
    error::Error,
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::tempdir;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct GenerationMetadata {
    common_version: Version,
    generator_version: Version,
}

const EXECUTABLE: &str = env!("CARGO_BIN_EXE_standardizer-generate-data");

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

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
}

fn assert_failure(output: &Output, expected: &str) {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.starts_with("standardizer-generate-data: "));
    assert!(stderr.contains(expected), "{stderr}");
}

#[test]
fn missing_command() -> Result<(), Box<dyn Error>> {
    let output = Command::new(EXECUTABLE).output()?;

    assert_failure(&output, "missing command");

    Ok(())
}

#[test]
fn unknown_command() -> Result<(), Box<dyn Error>> {
    let output = Command::new(EXECUTABLE).arg("unknown").output()?;

    assert_failure(&output, "unknown command");

    Ok(())
}

#[test]
fn check_missing_directory() -> Result<(), Box<dyn Error>> {
    let output = Command::new(EXECUTABLE).arg("check").output()?;

    assert_failure(&output, "missing RON directory");

    Ok(())
}

#[test]
fn check_excess_arguments() -> Result<(), Box<dyn Error>> {
    let output = Command::new(EXECUTABLE)
        .args(["check", "source", "extra"])
        .output()?;

    assert_failure(&output, "check expects exactly one RON directory");

    Ok(())
}

#[test]
fn generate_missing_source_directory() -> Result<(), Box<dyn Error>> {
    let output = Command::new(EXECUTABLE).arg("generate").output()?;

    assert_failure(&output, "missing RON directory");

    Ok(())
}

#[test]
fn generate_missing_output_directory() -> Result<(), Box<dyn Error>> {
    let output = Command::new(EXECUTABLE)
        .args(["generate", "source"])
        .output()?;

    assert_failure(&output, "missing BIN directory");

    Ok(())
}

#[test]
fn generate_excess_arguments() -> Result<(), Box<dyn Error>> {
    let output = Command::new(EXECUTABLE)
        .args(["generate", "source", "output", "extra"])
        .output()?;

    assert_failure(
        &output,
        "generate expects exactly one RON directory and one BIN directory",
    );

    Ok(())
}

#[test]
fn check_success() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;

    let output = Command::new(EXECUTABLE)
        .arg("check")
        .arg(directory.path())
        .output()?;

    assert_success(&output);
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read_dir(directory.path())?.count(), 2);

    Ok(())
}

#[test]
fn check_failure() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;

    let output = Command::new(EXECUTABLE)
        .arg("check")
        .arg(directory.path())
        .output()?;

    assert_failure(&output, "routing.ron");

    Ok(())
}

#[test]
fn generate_success() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let source = directory.path().join("RON source");
    let destination = directory.path().join("BIN output");

    write_sources(&source)?;

    let output = Command::new(EXECUTABLE)
        .arg("generate")
        .arg(&source)
        .arg(&destination)
        .output()?;

    assert_success(&output);

    let routing = Routing::decode(&fs::read(destination.join("routing.bin"))?)?;

    let tools = ToolConfiguration::decode(&fs::read(destination.join("tools.bin"))?)?;

    assert_eq!(routing.routes().len(), 1);
    assert_eq!(tools.tools().len(), 1);

    let metadata: GenerationMetadata = ron::from_str(std::str::from_utf8(&output.stdout)?)?;

    assert_eq!(
        metadata.common_version,
        Version::parse(Package::current().version())?
    );

    assert_eq!(
        metadata.generator_version,
        Version::parse(env!("CARGO_PKG_VERSION"))?
    );

    assert_eq!(fs::read_dir(&source)?.count(), 2);
    assert_eq!(fs::read_dir(&destination)?.count(), 2);

    Ok(())
}

#[test]
fn generate_in_source_directory() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    write_sources(directory.path())?;

    let output = Command::new(EXECUTABLE)
        .arg("generate")
        .arg(directory.path())
        .arg(directory.path())
        .output()?;

    assert_success(&output);

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

    Ok(())
}

#[test]
fn generate_failure() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let source = directory.path().join("source");
    let destination = directory.path().join("output");

    write_sources(&source)?;
    fs::write(source.join("tools.ron"), "invalid")?;

    let output = Command::new(EXECUTABLE)
        .arg("generate")
        .arg(&source)
        .arg(&destination)
        .output()?;

    assert_failure(&output, "tools.ron");
    assert!(!destination.exists());

    Ok(())
}
