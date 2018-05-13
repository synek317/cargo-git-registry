use std::path::{Path, PathBuf};
use directories::BaseDirs;
use failure::Error;
use regex::Regex;
use utils::lines::*;

pub fn global_cargo_config_path() -> PathBuf {
    BaseDirs::new()
        .home_dir()
        .join(".cargo")
        .join("config")
}

pub fn register_registry<P: AsRef<Path>>(config_path: P, registry_name: &str, url: &str) -> Result<(), Error> {
    rewrite_config_without_registry(
        config_path,
        registry_name,
        vec![
            &format!("[registries.{}]", registry_name),
            &format!("index = \"{}\"", url)
        ]
    )
}

pub fn unregister_registry<P: AsRef<Path>>(config_path: P, registry_name: &str) -> Result<(), Error> {
    rewrite_config_without_registry(
        config_path,
        registry_name,
        vec![]
    )
}

fn rewrite_config_without_registry<P: AsRef<Path>>(path: P, registry_name: &str, additional_lines: Vec<&String>) -> Result<(), Error> {
    let registry_section = registry_section(registry_name);
    let any_section      = Regex::new(r"^\s*\[").unwrap();
    let lines            = read_lines(&path)?;
    let before           = take_lines_until(&registry_section, &lines);
    let after            = skip_lines_until(&any_section, &lines, before.len() + 1);

    write_lines(
        &path,
        before
            .into_iter()
            .chain(after)
            .chain(additional_lines)
    )
}

fn registry_section(name: &str) -> Regex {
    Regex::new(&format!(r"^\s*\[\s*registries.{}\s*\]", name)).unwrap()
}
