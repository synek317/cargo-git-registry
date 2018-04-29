use std::io::{self, Read, BufRead};
use cmd_args::RemoveArgs;
use failure::{Error, ResultExt};
use utils::{KnownRepos, unregister_registry, global_cargo_config_path};
use git2;

pub fn remove(args: RemoveArgs) -> Result<(), Error> {
    let known_repos = KnownRepos::read()?;
    let (name, _path) = known_repos.find_or_err(&args.registry.unwrap_or(".".to_string()))?;

    unregister_registry(
        args.cargo_config_path.unwrap_or_else(global_cargo_config_path),
        &name
    )
}

