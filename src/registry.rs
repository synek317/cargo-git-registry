use std::path::PathBuf;
use url::Url;

struct Registry {
    pub name: String,
    pub path: PathBuf,
    pub url: Url
}
