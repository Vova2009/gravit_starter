use crate::jre::find_jre;
use crate::util::download_file;
use crate::CONFIG;
use anyhow::Result;
use dirs::data_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn launcher_exist() -> bool {
    let path = data_dir()
        .unwrap()
        .join(&CONFIG.project_name)
        .join("Launcher.jar");
    path.is_file()
}

pub fn run_launcher(jre_path: &Path) -> Result<()> {
    let mut jre_path = jre_path.join("bin").join("java.exe");
    let launcher_path = data_dir()
        .unwrap()
        .join(&CONFIG.project_name)
        .join("Launcher.jar");
    Command::new(&jre_path)
        .args(&["-jar", launcher_path.to_str().unwrap()])
        .output()?;
    Ok(())
}

pub fn download_launcher() -> Result<()> {
    let launcher_path = data_dir()
        .unwrap()
        .join(&CONFIG.project_name)
        .join("Launcher.jar");
    download_file(&CONFIG.launcher_url, &launcher_path)?;
    Ok(())
}
