use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::{create_dir_all, File};
use std::path::Path;
use failure::{Error, ResultExt};
use regex::Regex;

pub fn take_lines_until<'a>(pattern: &Regex, lines: &'a [String]) -> Vec<&'a String> {
    lines
        .iter()
        .take_while(|line| !pattern.is_match(&line))
        .collect()
}

pub fn skip_lines_until<'a>(pattern: &Regex, lines: &'a [String], lines_to_skip: usize) -> Vec<&'a String> {
    lines
        .iter()
        .skip(lines_to_skip)
        .skip_while(|line| !pattern.is_match(&line))
        .collect()
}

pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Error> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path).with_context(|_| format!("could not open file {}", path.display()))?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|_| format!("could not read file {}", path.display()))
        .map_err(Into::into)
}

pub fn write_lines<'a, P, L>(path: P, lines: L) -> Result<(), Error>
    where P: AsRef<Path>,
          L: Iterator<Item=&'a String>
{
    let path = path.as_ref();

    if let Some(dir) = path.parent() {
        create_dir_all(dir)
            .with_context(|_| format!("could not create directory {}", dir.display()))?;
    }

    let file       = File::create(path).with_context(|_| format!("could not open cargo config file {}", path.display()))?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)
            .with_context(|_| format!("could not write to cargo config file {}", path.display()))?;
    }

    Ok(())
}
