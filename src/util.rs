use anyhow::Result;
use http_req::request;
use std::fs::File;
use std::path::Path;
use std::{fs, io};
use url::Url;

pub fn download_file(url: &Url, path: &Path) -> Result<()> {
    let mut out = File::create(path)?;
    request::get(&url.to_string(), &mut out)?;
    Ok(())
}

pub fn get_pointer_width() -> i32 {
    use std::mem;
    use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO_u_s, SYSTEM_INFO};
    use winapi::um::winnt::{PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_INTEL};

    let mut system_info: SYSTEM_INFO = unsafe { mem::zeroed() };

    unsafe { GetNativeSystemInfo(&mut system_info) };

    let s: &SYSTEM_INFO_u_s = unsafe { system_info.u.s() };

    match s.wProcessorArchitecture {
        PROCESSOR_ARCHITECTURE_INTEL => 32,
        PROCESSOR_ARCHITECTURE_AMD64 => 64,
        _ => 0,
    }
}
pub fn extract_zip(zip: &Path, folder: &Path) -> Result<()> {
    let file = fs::File::open(&zip).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = folder.join(file.sanitized_name());
        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    Ok(())
}
