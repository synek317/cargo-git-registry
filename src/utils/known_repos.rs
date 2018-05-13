use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{create_dir_all, File};
use std::io::{self, BufWriter, Write};
use directories::ProjectDirs;
use failure::{Error, ResultExt};
use relative_path::RelativePath;
use utils::read_toml_file;
use toml;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KnownRepos {
    pub repos: HashMap<String, String> //name, path
}

impl KnownRepos {
    pub fn add<P: AsRef<Path>>(&mut self, name: &str, path: P) -> Result<(), Error> {
        let path = absolute_path(&path)?;

        self.repos.insert(name.to_string(), path.to_string_lossy().to_string());
        self.write()?;

        Ok(())

    }

    pub fn remove(&mut self, id: &str) -> Result<(), Error> {
        if let Some((name, _)) = self.find(id)? {
            self.repos.remove(&name);
            self.write()?;
        }

        Ok(())
    }

    pub fn find(&self, id: &str) -> Result<Option<(String, String)>, Error> {
        if let Some(path) = self.repos.get(id) {
            Ok(Some((id.to_string(), path.to_string())))
        }
        else {
            absolute_path(id)
                .map(|abs_path| abs_path.to_string_lossy().to_string())
                .map(|abs_path|
                    self.repos.iter()
                        .find(|&(_, v)| &*v == &*abs_path)
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                )
                .map_err(Into::into)
        }
    }

    pub fn find_or_err(&self, id: &str) -> Result<(String, String), Error> {
        self.find(id)?
            .ok_or_else(|| format_err!("Could not find any known registry {}.\nYou can list known repositories with `cargo git-repository list` or add new using `cargo git-repository remember <name> <path>", id))
    }

    pub fn read() -> Result<Self, Error> {
        let config_file_path = get_config_file_path()?;

        Ok(
            if config_file_path.exists() {
                read_toml_file(&config_file_path)?
            } else {
                KnownRepos::default()
            }
        )
    }

    pub fn write(&self) -> Result<(), Error> {
        let config_file_path = get_config_file_path()?;
        let file = File::create(&config_file_path)
            .with_context(|_| format!("could not create known repos  file {}", config_file_path.display()))?;
        let  bytes = toml::to_vec(self)
            .with_context(|_| "could not serialize known repos")?;

        BufWriter::new(file)
            .write_all(&bytes)
            .with_context(|_| format!("could not write known repos file {}", config_file_path.display()))
            .map_err(Into::into)
    }

}



fn get_config_file_path() -> Result<PathBuf, Error> {
    let dirs = ProjectDirs::from("com", "synek317", "cargo-git-registry");
    let config_dir = dirs.config_dir();
    let config_file_path = config_dir.join("repos.toml");

    create_dir_all(&config_dir)
        .with_context(|_| format!("could not create config directory {}", config_dir.display()))?;

    Ok(config_file_path)
}

fn absolute_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();

    Ok(
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            RelativePath::new(current_dir()?.to_string_lossy().as_ref())
                .join_normalized(path.to_string_lossy().as_ref())
                .to_path("/")
                .to_path_buf()
        }
    )
}
