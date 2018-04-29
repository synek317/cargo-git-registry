use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::{BufRead, BufReader, BufWriter, Write};
use directories::BaseDirs;
use failure::{Error, ResultExt};
use regex::Regex;

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
            &registry_section(registry_name),
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
    let path             = path.as_ref();
    let registry_section = registry_section(registry_name);
    let any_section      = Regex::new(r"^\s*\[").unwrap();
    let lines            = read_lines(path)?;
    let before           = take_lines_until(&registry_section, &lines);
    let after            = skip_lines_until(&any_section, &lines, before.len() + 1);

    if let Some(dir) = path.parent() {
        create_dir_all(dir)
            .with_context(|_| format!("could not create directory {}", dir.display()))?;
    }

    let file             = File::create(path).with_context(|_| format!("could not open cargo config file {}", path.display()))?;
    let mut writer       = BufWriter::new(file);

    for line in before
        .into_iter()
        .chain(after)
        .chain(additional_lines)
    {
        writeln!(writer, "{}", line)
            .with_context(|_| format!("could not write to cargo config file {}", path.display()))?;
    }

    Ok(())
}

fn registry_section(name: &str) -> String {
    format!("[registries.{}]", name)
}

fn take_lines_until<'a>(pattern: &str, lines: &'a [String]) -> Vec<&'a String> {
    lines
        .iter()
        .take_while(|line| line.matches(&pattern).next().is_none())
        .collect()
}

fn skip_lines_until<'a>(pattern: &Regex, lines: &'a [String], lines_to_skip: usize) -> Vec<&'a String> {
    lines
        .iter()
        .skip(lines_to_skip)
        .skip_while(|line| !pattern.is_match(&line))
        .collect()
}

fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Error> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path).with_context(|_| format!("could not open cargo config file {}", path.display()))?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|_| format!("could not read cargo config file {}", path.display()))
        .map_err(Into::into)
}
