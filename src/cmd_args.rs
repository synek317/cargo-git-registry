use std::path::PathBuf;
use url::Url;

#[derive(StructOpt, Debug)]
pub enum CmdArgs {
    #[structopt(name = "init", about = "Initializes new registry in given directory")]
    Init(InitArgs),

    #[structopt(name = "clone", help = "Clones the registry from git repository")]
    Clone(CloneArgs),

    #[structopt(name = "update", help = "Updates git registry. Requires clean working directory, performs git pull")]
    Update(UpdateArgs),

    #[structopt(name = "publish", help = "Publishes crate to git registry. Requires clean working directory, preforms git push")]
    Publish(PublishArgs),

    #[structopt(name = "list", help = "Lists known alternative repositories")]
    List,

    #[structopt(name = "remember", help = "Remembers the path for the repository name")]
    Remember(RememberArgs),

    #[structopt(name = "forget", help = "Forgets the connection between the repository name and path")]
    Forget(ForgetArgs),

    #[structopt(name = "use", help = "Adds the registry to cargo config file")]
    Use(UseArgs),

    #[structopt(name = "remove", help = "Removes the repository from cargo's config")]
    Remove(RemoveArgs)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "init", help = "Converts native Totalstay files to ARIZ.")]
pub struct InitArgs {
    #[structopt(help = "Name of the registry. If not given, directory name is used")]
    pub name: Option<String>,

    #[structopt(help = "When given, repository will be cloned and initialized (including git push)")]
    pub url: Option<Url>,

    #[structopt(short = "d", long = "dir", help = "Path to directory where registry should be created", parse(from_os_str))]
    pub path: PathBuf
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
    #[structopt(long = "manifest-path", help = "Path to the package manifest to publish", parse(from_os_str))]
    pub manifest_path: Option<PathBuf>,

    #[structopt(help = "Name of or path to the registry")]
    pub registry: Option<String>
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
    pub registry: Option<String>,

    #[structopt(long = "cargo-config-path", help = "Path to .cargo directory containing config file. If not given, global cargo config directory is used, e.g. &HOME/.cargo/config", parse(from_os_str))]
    pub cargo_config_path: Option<PathBuf>
}
