use crate::util::download_file;
use crate::CONFIG;
use anyhow::Result;
use dirs::data_dir;
use std::process::Command;

pub fn launcher_exist() -> bool {
    let path = data_dir()
        .unwrap()
        .join(&CONFIG.project_name)
        .join("Launcher.jar");
    path.is_file()
}

pub fn run_launcher() -> Result<()> {
    let jre_path = data_dir()
        .unwrap()
        .join(&CONFIG.project_name)
        .join("launcher-jre")
        .join("bin");
    let launcher_path = data_dir()
        .unwrap()
        .join(&CONFIG.project_name)
        .join("Launcher.jar");
    Command::new("java.exe")
        .current_dir(&jre_path)
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
