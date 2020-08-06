use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) project_name: String,
    pub(crate) launcher_url: Url,
    pub(crate) jre_urls: Jre,
    pub(crate) title: String,
}

#[derive(Deserialize)]
pub struct Jre {
    pub(crate) x32: Url,
    pub(crate) x64: Url,
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("../config.json")).unwrap()
    }
}
