# Standardizer Generate Data

`standardizer-generate-data` generates the binary data used by Standardizer.

It is tooling associated with Standardizer and is published on crates.io.

## Installation

```sh
cargo install standardizer-generate-data --locked
```

## Usage

```sh
standardizer-generate-data <ron-directory> <bin-directory>
```

The RON directory must contain:

- `routing.ron`
- `tools.ron`

The generated files are written to the BIN directory:

- `routing.bin`
- `tools.bin`

For the Standardizer data repositories:

```sh
standardizer-generate-data standardizer-data-ron standardizer-data-bin
```

## Development

```sh
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
