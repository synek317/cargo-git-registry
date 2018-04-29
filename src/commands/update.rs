use std::io::{self, Read, BufRead};
use cmd_args::UpdateArgs;
use failure::{Error, ResultExt};
use utils::{KnownRepos, register_registry, global_cargo_config_path, get_remote_name};
use git2;

pub fn update(args: UpdateArgs) -> Result<(), Error> {
    let known_repos = KnownRepos::read()?;
    let (_name, path) = known_repos.find_or_err(&args.registry.unwrap_or(".".to_string()))?;
    let repo = git2::Repository::open(&path).with_context(|_| format!("could not open git repository"))?;

    if repo.state() != git2::RepositoryState::Clean {
        bail!("working directory must be clean in {}", path)
    }

    let remote_name = get_remote_name(&repo)?;

    //Warning: I'm totally not sure if I'm doing it right

    repo.find_remote(&remote_name)
        .with_context(|_| format!("could not find remote {}", remote_name))?
        .fetch(&[], None, None)?;

    let fetch_head_oid = repo.refname_to_id("FETCH_HEAD")?;
    let fetch_head_obj = repo.find_object(fetch_head_oid, None)?;

    repo.reset(&fetch_head_obj, git2::ResetType::Hard, None)?;

    Ok(())
}
