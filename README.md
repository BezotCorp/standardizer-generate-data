# Standardizer Generate Data

`standardizer-generate-data` validates and generates the binary data used by Standardizer.

It is tooling associated with Standardizer and is published on crates.io.

The shared data structures, validation rules, RON deserialization and
binary encoding are managed exclusively by standardizer-common.

## Installation

```sh
cargo install standardizer-generate-data --locked
```

## Check

Validate the RON data without generating binary files:

```sh
standardizer-generate-data check <ron-directory>
```

The RON directory must contain:

- `routing.ron`
- `tools.ron`

## Generate

Validate the RON data and generate the corresponding binary files:

```sh
standardizer-generate-data generate <ron-directory> <bin-directory>
```

The generated files are written to the BIN directory:

- `routing.bin`
- `tools.bin`

From the Standardizer repository:

```sh
standardizer-generate-data check data
standardizer-generate-data generate data data
```

## Automation

The `generate` command exposes its generation result on standard output
using RON:

```ron
(
    common_version: "2.3.0",
    generator_version: "1.1.0",
)
```

Both fields represent semantic versions:

- `common_version`: version of the linked `standardizer-common` library.
- `generator_version`: version of the running generator.

The versions above are illustrative.

A successful generation returns exit code zero.
Errors are reported on standard error with a non-zero exit code.

The caller is responsible for selecting the generator version and
providing the input and output directories.

The input and output directories may be identical. RON source files
are not modified by the generator.

Generated binary files are build artifacts. The caller must not use
them if generation fails.

## Development

```sh
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
