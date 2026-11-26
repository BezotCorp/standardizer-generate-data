use semver::Version;
use serde::Serialize;
use standardizer_common::introspection::Package;

#[derive(Serialize)]
pub(crate) struct GenerationResult {
    common_version: Version,
    generator_version: Version,
}

impl GenerationResult {
    pub(crate) fn new() -> Result<Self, semver::Error> {
        Ok(Self {
            common_version: Version::parse(Package::current().version())?,
            generator_version: Version::parse(env!("CARGO_PKG_VERSION"))?,
        })
    }
}
