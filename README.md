# Standardizer Generate Data

`standardizer-generate-data` validates and generates the binary data used by Standardizer.

It is tooling associated with Standardizer and is published on crates.io.

## Installation

~~~sh
cargo install standardizer-generate-data --locked
~~~

## Check

Validate the RON data without generating binary files:

~~~sh
standardizer-generate-data check <ron-directory>
~~~

The RON directory must contain:

- `routing.ron`
- `tools.ron`

## Generate

Validate the RON data and generate the corresponding binary files:

~~~sh
standardizer-generate-data generate <ron-directory> <bin-directory>
~~~

The generated files are written to the BIN directory:

- `routing.bin`
- `tools.bin`

For the Standardizer data repositories:

~~~sh
standardizer-generate-data check standardizer-data-ron
standardizer-generate-data generate standardizer-data-ron standardizer-data-bin
~~~

## Development

~~~sh
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
~~~

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
