use cmd_args::UpdateArgs;
use failure::{Error, ResultExt};
use utils::{KnownRepos, git_reset_origin};
use git2;

pub fn update(args: UpdateArgs) -> Result<(), Error> {
    let known_repos = KnownRepos::read()?;
    let (_name, path) = known_repos.find_or_err(&args.registry.unwrap_or(".".to_string()))?;
    let repo = git2::Repository::open(&path).with_context(|_| format!("could not open git repository"))?;

    if repo.state() != git2::RepositoryState::Clean {
        bail!("working directory must be clean in {}", path)
    }

    git_reset_origin(&repo)
}
