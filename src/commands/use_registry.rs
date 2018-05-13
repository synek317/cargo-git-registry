use std::env::current_dir;
use cmd_args::UseArgs;
use failure::{Error, ResultExt};
use utils::{KnownRepos, register_registry, global_cargo_config_path, get_origin_url};
use git2;

pub fn use_registry(args: UseArgs) -> Result<(), Error> {
    let known_repos       = KnownRepos::read()?;
    let (name, path)      = known_repos.find_or_err(&args.registry)?;
    let repo              = git2::Repository::open(&path).with_context(|_| format!("could not open git repository"))?;
    let url               = get_origin_url(&repo)?;
    let cargo_config_path = match (args.global, args.cargo_config_path) {
        (true,  Some(_))    => bail!("--cargo-config-path and --global arguments are mutually exclusive"),
        (true,  None)       => global_cargo_config_path(),
        (false, None)       => current_dir()?,
        (false, Some(path)) => path,
    };

    register_registry(cargo_config_path, &name, &url)
}

