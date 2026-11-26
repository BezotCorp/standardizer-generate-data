use crate::{dataset::Dataset, generation_result::GenerationResult};
use standardizer_common::{routes::Routing, tools::ToolConfiguration};
use std::{error::Error, fs, path::Path};

const ROUTING_RON_FILE: &str = "routing.ron";
const TOOLS_RON_FILE: &str = "tools.ron";
const ROUTING_BIN_FILE: &str = "routing.bin";
const TOOLS_BIN_FILE: &str = "tools.bin";

pub(crate) struct Generator;

impl Generator {
    pub(crate) fn check(ron_directory: &Path) -> Result<(), Box<dyn Error>> {
        Self::read(ron_directory)?;
        Ok(())
    }

    pub(crate) fn generate(
        ron_directory: &Path,
        bin_directory: &Path,
    ) -> Result<GenerationResult, Box<dyn Error>> {
        let result = GenerationResult::new()?;
        let (routing, tools) = Self::read(ron_directory)?;
        let routing_binary = Dataset::encode(&routing)?;
        let tools_binary = Dataset::encode(&tools)?;

        fs::create_dir_all(bin_directory)?;
        fs::write(bin_directory.join(ROUTING_BIN_FILE), routing_binary)?;
        fs::write(bin_directory.join(TOOLS_BIN_FILE), tools_binary)?;

        Ok(result)
    }

    fn read(ron_directory: &Path) -> Result<(Routing, ToolConfiguration), Box<dyn Error>> {
        let routing_path = ron_directory.join(ROUTING_RON_FILE);
        let tools_path = ron_directory.join(TOOLS_RON_FILE);

        let routing: Routing = Dataset::read(&routing_path)?;

        routing
            .validate()
            .map_err(|error| format!("{}: {error}", routing_path.display()))?;

        let tools: ToolConfiguration = Dataset::read(&tools_path)?;

        tools
            .validate()
            .map_err(|error| format!("{}: {error}", tools_path.display()))?;

        Ok((routing, tools))
    }
}
