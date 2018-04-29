use failure::Error;
use utils::KnownRepos;

pub fn list() -> Result<(), Error> {
    let known_repos = KnownRepos::read()?;

    if known_repos.repos.len() == 0 {
        println!("No git registries found. You can add one using `cargo git-registry remember <name> <path>`");
    } else {
        println!("List of available git registries:");

        for (index, (name, path)) in known_repos.repos.iter().enumerate() {
            println!("{}. {} in {}", index + 1, name, path);
        }

        println!();
        println!("Manage git registries using `cargo git-registry remember` and `cargo git-registry forget`");
    }

    Ok(())
}
