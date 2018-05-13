use std::path::PathBuf;
use url::Url;

#[derive(StructOpt, Debug)]
pub enum CmdArgs {
    #[structopt(name = "init", about = "Initializes new registry in given directory")]
    Init(InitArgs),

    #[structopt(name = "clone", about = "Clones the registry from git repository")]
    Clone(CloneArgs),

    #[structopt(name = "update", about = "Updates git registry. Requires clean working directory, performs git pull")]
    Update(UpdateArgs),

    #[structopt(name = "publish", about = "Publishes crate to git registry. Requires clean working directory, preforms git push")]
    Publish(PublishArgs),

    #[structopt(name = "list", about = "Lists known alternative repositories")]
    List,

    #[structopt(name = "remember", about = "Remembers the path for the repository name")]
    Remember(RememberArgs),

    #[structopt(name = "forget", about = "Forgets the connection between the repository name and path")]
    Forget(ForgetArgs),

    #[structopt(name = "use", about = "Adds the registry to cargo config file")]
    Use(UseArgs),

    #[structopt(name = "remove", about = "Removes the repository from cargo's config")]
    Remove(RemoveArgs)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "init", help = "Converts native Totalstay files to ARIZ.")]
pub struct InitArgs {
    #[structopt(help = "Name of or path to the registry. If not given, current directory is used")]
    pub registry: Option<String>
}

#[derive(StructOpt, Debug)]
pub struct CloneArgs {
    #[structopt(help = "Url to the git repository with the registry")]
    pub url: Url,

    #[structopt(short = "d", long = "dir", help = "Path to directory where registry should be created", parse(from_os_str))]
    pub path: Option<PathBuf>,

    #[structopt(short = "n", long = "name", help = "Name of the registry. If not given, name of the repository is used")]
    pub name: Option<String>,

    #[structopt(short = "g", long = "global", help = "If true, adds registry to global cargo config")]
    pub global: bool
}

#[derive(StructOpt, Debug)]
pub struct UpdateArgs {
    #[structopt(help = "Name of or path to the registry. If not given, current directory is used")]
    pub registry: Option<String>
}

#[derive(StructOpt, Debug)]
pub struct PublishArgs {
    #[structopt(long = "manifest-path", help = "Path to the package manifest to publish. If not given, current directory is used", parse(from_os_str))]
    pub manifest_path: Option<PathBuf>,

    #[structopt(help = "Name of or path to the registry")]
    pub registry: String,

    #[structopt(long = "allow-dirty", help = "Allows packaging and publishing crate from non-clean git repository; implies --no-git")]
    pub allow_dirty: bool,

    #[structopt(short = "v", long = "version", help = "Updates Cargo.toml to specific version (includes commit and push")]
    pub version: Option<String>,

    #[structopt(long = "next-major", help = "Updates Cargo.toml to next major version (includes commit and push")]
    pub next_major: bool,

    #[structopt(long = "next-minor", help = "Updates Cargo.toml to next minor version (includes commit and push")]
    pub next_minor: bool,

    #[structopt(long = "next-patch", help = "Updates Cargo.toml to next patch version (includes commit and push")]
    pub next_patch: bool,

    #[structopt(long = "no-tag", help = "Prevents from creating and pushing git tag")]
    pub no_tag: bool,

    #[structopt(long = "no-git", help = "Prevents any git action on the package; implies --no-tag")]
    pub no_git: bool,
}

#[derive(StructOpt, Debug)]
pub struct RememberArgs {
    #[structopt(help = "name of the registry")]
    pub name: String,

    #[structopt(help = "path to the registry", parse(from_os_str))]
    pub path: PathBuf
}

#[derive(StructOpt, Debug)]
pub struct ForgetArgs {
    #[structopt(help = "name of or path to the registry. If not given, current directory is used")]
    pub registry: Option<String>
}

#[derive(StructOpt, Debug)]
pub struct RemoveArgs {
    #[structopt(help = "name of or path to the registry. If not given, current directory is used")]
    pub registry: Option<String>,

    #[structopt(long = "cargo-config-path", help = "Path to .cargo directory containing config file. If not given, global cargo config directory is used, e.g. &HOME/.cargo/config", parse(from_os_str))]
    pub cargo_config_path: Option<PathBuf>
}

#[derive(StructOpt, Debug)]
pub struct UseArgs {
    #[structopt(help = "name of or path to the registry. If not given, current directory is used")]
    pub registry: String,

    #[structopt(long = "cargo-config-path", help = "Path to .cargo directory containing config file. If not given, local cargo config directory is used, e.g. ./.cargo/config", parse(from_os_str))]
    pub cargo_config_path: Option<PathBuf>,

    #[structopt(short = "g", long = "global", help = "If true, adds registry to global cargo config")]
    pub global: bool
}
