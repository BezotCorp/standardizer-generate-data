# Standardizer Generate Data

`standardizer-generate-data` validates and generates the binary data used by Standardizer.

It is tooling associated with Standardizer and is published on crates.io.

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

For the Standardizer data repositories:

```sh
standardizer-generate-data check standardizer-data-ron
standardizer-generate-data generate standardizer-data-ron standardizer-data-bin
```

## Automation

The `generate` command exposes its generation result on standard output
using RON:

```ron
(
    common_version: "2.2.0",
    generator_version: "1.0.1",
)
```

Both fields represent semantic versions:

- `common_version`: version of the linked `standardizer-common` library.
- `generator_version`: version of the running generator.

The versions above are illustrative.

A successful generation returns exit code zero.
Errors are reported on standard error with a non-zero exit code.

The caller is responsible for selecting the data version, maintaining
provenance and publishing the generated data.

The caller should use an isolated output directory and discard its
contents if generation fails.

## Development

```sh
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
