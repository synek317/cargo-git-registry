use std::io::{self, Read};
use failure::{Error, ResultExt};
use git2;

pub fn get_remote_url(repo: &git2::Repository) -> Result<String, Error> {
    let name = get_remote_name(repo)?;

    repo
        .find_remote(&name)
        .with_context(|_| format!("could not find remote {}", name))
        .map_err(Into::into)
        .and_then(|r| r.url().map(|u| u.to_string()).ok_or_else(|| format_err!("could not find url for remote {}", name)))
}

pub fn get_remote_name(repo: &git2::Repository) -> Result<String, Error> {
    let remotes = repo.remotes().with_context(|_| format!("cannot list remotes in {}", repo.path().display()))?;

    Ok(
        if remotes.len() == 0 {
            bail!("No remotes found in {}", repo.path().display());
        }
        else if remotes.len() == 1 {
            remotes.get(0).unwrap().to_string()
        }
        else {
            println!("Choose url:");

            for (index, remote) in remotes.iter().enumerate() {
                let name = remote.unwrap_or("");
                let url = repo.find_remote(name).ok().and_then(|r| r.url().map(|u| u.to_string())).unwrap_or("".to_string());

                println!("{}. {} ({})", index + 1, url, name);
            }

            println!("0. Abort");
            let mut number: Option<usize>;

            while {
                let mut buffer = String::new();

                println!("Choice [0-{}]: ", remotes.len());
                io::stdin().read_line(&mut buffer)?;

                number = buffer.trim().parse().ok();

                number.map(|n| n > remotes.len()).unwrap_or(true)
            } {}

            if number == Some(0) {
                bail!("Aborted");
            } else {
                remotes.get(number.unwrap() - 1).unwrap().to_string()
            }
        }
    )
}
