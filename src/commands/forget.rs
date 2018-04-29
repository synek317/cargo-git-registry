use cmd_args::ForgetArgs;
use failure::Error;
use utils::KnownRepos;

pub fn forget(args: ForgetArgs) -> Result<(), Error> {
    let mut known_repos = KnownRepos::read()?;

    known_repos.remove(&args.registry.unwrap_or(".".to_string()))?;

    Ok(())
}
