use std::io::{BufReader, Read};
use std::fs::File;
use std::path::Path;
use failure::{Error, ResultExt};
use serde::de;
use toml;

pub fn read_toml_file<P, T>(path: P) -> Result<T, Error>
    where T: de::DeserializeOwned   ,
          P: AsRef<Path>
{
    let path = path.as_ref();
    let file = File::open(path)
        .with_context(|_| format!("could not open toml file {}", path.display()))?;
    let mut bytes = Vec::with_capacity(10240);
    let mut reader = BufReader::new(file);

    reader.read_to_end(&mut bytes)
        .with_context(|_| format!("could not read toml file {}", path.display()))?;

    let toml = toml::from_slice(&bytes)
        .with_context(|_| format!("invalid toml file {}", path.display()))?;

    Ok(toml)
}
