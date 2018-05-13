use std::env::current_dir;
use std::path::{Path, PathBuf};
use cargo::ops::{package, generate_index_metadata, PackageOpts};
use cargo::Config;
use cargo::core::Workspace;
use cmd_args::PublishArgs;
use failure::{Error, ResultExt};
use git2;
use regex::Regex;
use semver::Version;
use toml;
use utils::*;

pub fn publish(args: PublishArgs) -> Result<(), Error> {
    let known_repos   = KnownRepos::read()?;
    let (name, path)  = known_repos.find_or_err(&args.registry)?;
    let registry_repo = git2::Repository::open(&path).with_context(|_| format!("could not open git repository"))?;

    if registry_repo.state() != git2::RepositoryState::Clean {
        bail!("working directory must be clean in {}", path)
    }

    let manifest_path = args.manifest_path.clone().unwrap_or(current_dir()?);
    let manifest_path = if manifest_path.file_name() == Some("Cargo.toml".as_ref()) { manifest_path } else { manifest_path.join("Cargo.toml") };

    let cargo_config = Config::default()?;
    let package_opts = PackageOpts {
        config:         &cargo_config,
        list:           false,
        check_metadata: true,
        allow_dirty:    args.allow_dirty,
        verify:         true,
        jobs:           None,
        target:         None,
        registry:       None,
    };

    update_version(&args, &manifest_path, &cargo_config)?;

    let workspace = Workspace::new(&manifest_path, &cargo_config)?;

    package(&workspace, &package_opts)?;

    let registry_package = generate_index_metadata(&workspace)?;

    //cp package file
    //file_write
    //commit_and_push
    Ok(())
}

//fn get_repo(args: &PublishArgs, regirepo_path) -> Result<Option<git2::Repository>, Error> {
//    if args.no_git {
//        return Ok(None);
//    }
//
//    let repo = git2::Repository::open(&path).with_context(|_| format!("could not open git repository"))?;
//
//    if repo.state() != git2::RepositoryState::Clean && !args.allow_dirty {
//        bail!("working directory must be clean in {}", path)
//    }
//
//    Ok(Some(repo))
//}

fn update_version<P: AsRef<Path>>(args: &PublishArgs, manifest_path: P, cargo_config: &Config) -> Result<(), Error> {
    if [args.version.is_some(), args.next_major, args.next_minor, args.next_patch]
        .iter()
        .fold(0, |acc, b| if *b { acc+1 } else { acc }) > 1
    {
        bail!("version can be specified only once using --version VERSION, --next-major, --next-minor, --next-patch")
    }

    if args.no_git && !args.allow_dirty {
        bail!("updating version requires either git action or --allow-dirty")
    }

    let workspace = Workspace::new(manifest_path.as_ref(), &cargo_config)?;
    let current_version = workspace
        .current()?
        .manifest()
        .version();

    let new_version = match args.version {
        Some(ref v) => v.clone(),
        None =>  {
            let mut new_version = current_version.clone();

            if args.next_major {
                new_version.increment_major();
            } else if args.next_minor {
                new_version.increment_minor();
            } else if args.next_patch {
                new_version.increment_patch();
            } else {
                return Ok(())
            }

            new_version.to_string()
        }
    };

    let lines              = read_lines(&manifest_path)?;
    let version_line_index = find_version_line_index(&lines)?;

    write_lines(
        &manifest_path,
        lines
            .iter()
            .take(version_line_index)
            .chain(&[format!(r#"version = "{}""#, new_version)])
            .chain(lines.iter().skip(version_line_index + 1))
    )?;

    println!("Updated version from {} to {}", current_version, new_version);

    if !args.no_git {
        let repo = manifest_path.as_ref()
            .parent()
            .ok_or_else(|| format_err!("could not find crate git repository"))
            .and_then(|dir| git2::Repository::open(dir)
                .with_context(|_| format!("could not open git repository"))
                .map_err(Into::into)
            )?;

        commit_and_push(&repo, || Ok((
            format!("Update crate version to {}", new_version),
            vec![
                manifest_path.as_ref().to_path_buf()
            ]
        )));
    }

    Ok(())
}

fn find_version_line_index(lines: &[String]) -> Result<usize, Error> {
    let package_section = Regex::new(r"^\s*\[\s*package\s*\]").unwrap();
    let any_section     = Regex::new(r"^\s*\[").unwrap();
    let version_line    = Regex::new(r"^\s*version\s*=").unwrap();

    for i in 0..lines.len() {
        if package_section.is_match(&lines[i]) {
            for j in i + 1..lines.len() {
                if version_line.is_match(&lines[j]) {
                    return Ok(j)
                }

                if any_section.is_match(&lines[j]) {
                    bail!("could not find version in package manifest file 1")
                }
            }
        }
    }

    bail!("could not find version in package manifest file")
}
