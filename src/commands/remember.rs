use cmd_args::RememberArgs;
use failure::Error;
use utils::KnownRepos;

pub fn remember(args: RememberArgs) -> Result<(), Error> {
    let mut known_repos = KnownRepos::read()?;

    known_repos.add(&args.name, args.path)?;

    Ok(())
}
