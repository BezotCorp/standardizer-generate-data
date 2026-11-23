use bincode_next::{Encode, config, encode_to_vec};
use serde::de::DeserializeOwned;
use std::{error::Error, fs, path::Path};

pub(crate) struct Dataset;

impl Dataset {
    pub(crate) fn read<T>(path: &Path) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let source = fs::read_to_string(path)?;

        ron::from_str(&source).map_err(|error| format!("{}: {error}", path.display()).into())
    }

    pub(crate) fn encode<T>(value: &T) -> Result<Vec<u8>, Box<dyn Error>>
    where
        T: Encode,
    {
        Ok(encode_to_vec(value, config::standard())?)
    }
}
