use std::env::current_dir;
use failure::{Error, ResultExt};
use git2;
use cmd_args::CloneArgs;
use utils::{KnownRepos, global_cargo_config_path, register_registry};

pub fn clone(args: CloneArgs) -> Result<(), Error> {
    let name = match args.name {
        Some(ref name) => name,
        None => {
            let tmp_name = args.url
                .path_segments()
                .and_then(|segments| segments.filter(|s| !s.is_empty()).last())
                .ok_or_else(|| format_err!("could not find name of the repository: {}", args.url))?;

            &tmp_name[0..tmp_name.rfind('.').unwrap_or(tmp_name.len())]
        }
    };
    let url = args.url.as_str();
    let dir = args.path.unwrap_or(current_dir()?);

    println!("Cloning registry '{}' from {} to {}...", name, url, dir.display());

    let repo = git2::Repository::clone(url, &dir).with_context(|_| format!("could not clone repository"))?;

    let mut known_repos = KnownRepos::read()?;

    known_repos.add(name, &dir)?;

    if args.global {
        register_registry(global_cargo_config_path(), name, &url)?;
    }

    Ok(())
}
