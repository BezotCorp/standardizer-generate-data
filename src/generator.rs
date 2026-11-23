use crate::dataset::Dataset;
use standardizer_common::{routes::Routing, tools::ToolConfiguration};
use std::{error::Error, fs, path::Path};

const ROUTING_FILE: &str = "routing.ron";
const TOOLS_FILE: &str = "tools.ron";

pub(crate) struct Generator;

impl Generator {
    pub(crate) fn check(ron_directory: &Path) -> Result<(), Box<dyn Error>> {
        Self::read(ron_directory)?;
        Ok(())
    }

    pub(crate) fn generate(
        ron_directory: &Path,
        bin_directory: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let (routing, tools) = Self::read(ron_directory)?;
        let routing_binary = Dataset::encode(&routing)?;
        let tools_binary = Dataset::encode(&tools)?;
        fs::create_dir_all(bin_directory)?;
        fs::write(bin_directory.join(ROUTING_FILE), routing_binary)?;
        fs::write(bin_directory.join(TOOLS_FILE), tools_binary)?;
        Ok(())
    }

    fn read(ron_directory: &Path) -> Result<(Routing, ToolConfiguration), Box<dyn Error>> {
        let routing_path = ron_directory.join(ROUTING_FILE);
        let tools_path = ron_directory.join(TOOLS_FILE);
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
