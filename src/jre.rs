use crate::util::{download_file, extract_zip, get_pointer_width};
use crate::CONFIG;
use anyhow::Result;
use dirs::data_dir;
use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn get_jre() -> Option<PathBuf> {
    let mut dir = data_dir().unwrap();
    dir.push(&CONFIG.project_name);
    dir.push("launcher-jre");
    if dir.is_dir() {
        Some(dir)
    } else if CONFIG.check_jre {
        find_jre()
    } else {
        None
    }
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

pub fn find_jre() -> Option<PathBuf> {
    Command::new("java")
        .arg("-XshowSettings:properties")
        .arg("-version")
        .output()
        .ok()
        .and_then(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stdout.lines().chain(stderr.lines()) {
                if line.contains("java.home") {
                    let pos = line.find('=').unwrap() + 1;
                    let path = line[pos..].trim();
                    let buf = PathBuf::from(path);
                    return if buf.join("lib").join("javafx.properties").is_file() {
                        Some(buf)
                    } else {
                        None
                    };
                }
            }
            None
        })
}
