use std::io::{self, Read, BufRead};
use cmd_args::UseArgs;
use failure::{Error, ResultExt};
use utils::{KnownRepos, register_registry, global_cargo_config_path, get_remote_url};
use git2;

pub fn use_registry(args: UseArgs) -> Result<(), Error> {
    let known_repos = KnownRepos::read()?;
    let (name, path) = known_repos.find_or_err(&args.registry.unwrap_or(".".to_string()))?;
    let repo = git2::Repository::open(&path).with_context(|_| format!("could not open git repository"))?;
    let url = get_remote_url(&repo)?;

    register_registry(
        args.cargo_config_path.unwrap_or_else(global_cargo_config_path),
        &name,
        &url
    )
}

