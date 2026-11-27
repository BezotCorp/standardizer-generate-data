use crate::generation_result::GenerationResult;
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
        let routing_binary = routing.encode()?;
        let tools_binary = tools.encode()?;

        fs::create_dir_all(bin_directory)?;
        fs::write(bin_directory.join(ROUTING_BIN_FILE), routing_binary)?;
        fs::write(bin_directory.join(TOOLS_BIN_FILE), tools_binary)?;

        Ok(result)
    }

    fn read(ron_directory: &Path) -> Result<(Routing, ToolConfiguration), Box<dyn Error>> {
        let routing_path = ron_directory.join(ROUTING_RON_FILE);
        let tools_path = ron_directory.join(TOOLS_RON_FILE);

        let routing_source = fs::read_to_string(&routing_path)
            .map_err(|error| format!("{}: {error}", routing_path.display()))?;

        let routing = Routing::from_ron(&routing_source)
            .map_err(|error| format!("{}: {error}", routing_path.display()))?;

        let tools_source = fs::read_to_string(&tools_path)
            .map_err(|error| format!("{}: {error}", tools_path.display()))?;

        let tools = ToolConfiguration::from_ron(&tools_source)
            .map_err(|error| format!("{}: {error}", tools_path.display()))?;

        Ok((routing, tools))
    }
}
