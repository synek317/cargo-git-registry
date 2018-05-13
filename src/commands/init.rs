use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use cmd_args::InitArgs;
use failure::{Error, ResultExt};
use git2;
use utils::{get_origin_url, KnownRepos};

pub fn init(args: InitArgs) -> Result<(), Error> {
    let known_repos   = KnownRepos::read()?;
    let (_name, path) = known_repos.find_or_err(&args.registry.unwrap_or(".".to_string()))?;
    let repo          = git2::Repository::open(&path).with_context(|_| format!("could not open git repository"))?;
    let config_path   = PathBuf::from(path).join("config.json");
    let mut url       = get_origin_url(&repo)?;
    let len           = url.len();

    if url.ends_with(".git") {
        url.truncate(len - 4);
    }

    File::create(config_path)
        .and_then(|mut f| writeln!(f, r#"
{{
    "dl": "{}/raw/master/crates"
}}
        "#, url))?;

    Ok(())
}
