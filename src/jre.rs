use crate::util::{download_file, extract_zip, get_pointer_width};
use crate::CONFIG;
use anyhow::Result;
use dirs::data_dir;
use std::fs;
use std::process::exit;

pub fn jre_exist() -> bool {
    let mut dir = data_dir().unwrap();
    dir.push(&CONFIG.project_name);
    dir.push("launcher-jre");
    dir.is_dir()
}

pub fn download_jre() -> Result<()> {
    let jre = match get_pointer_width() {
        64 => &CONFIG.jre_urls.x64,
        32 => &CONFIG.jre_urls.x32,
        _ => {
            unreachable!();
        }
    };
    let data_dir = data_dir().unwrap();
    fs::create_dir_all(data_dir.join(&CONFIG.project_name))?;
    let mut zip_path = data_dir.join(&CONFIG.project_name).join("jre.zip");
    download_file(jre, &zip_path)?;
    Ok(())
}

pub fn extract_jre() -> Result<()> {
    let data_dir = data_dir().unwrap();
    let mut zip_path = data_dir.join(&CONFIG.project_name).join("jre.zip");
    let mut dir_path = data_dir.join(&CONFIG.project_name).join("launcher-jre");
    extract_zip(&zip_path, &dir_path)?;
    fs::remove_file(zip_path)?;
    Ok(())
}
