// Adapted from github.com/dvdsk/Book-safe/blob/main/src/report.rs,
// with permission

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use color_eyre::eyre::{Context, Result};

#[cfg(target_arch = "arm")]
pub const DIR: &str = "/home/root/.local/share/remarkable/xochitl";
#[cfg(not(target_arch = "arm"))]
pub const DIR: &str = "output";

const REPORT_UUID: &str = "56f3ee9d-413b-4651-ac97-f9a9932a760a";

pub fn create_metadata_files(n_pages: usize) -> Result<()> {
    log::info!("report uuid: {REPORT_UUID} (constant)");
    let path = Path::new(DIR).join(REPORT_UUID);

    fs::write(path.with_extension("content"), content(n_pages))?;
    fs::write(path.with_extension("metadata"), metadata())?;
    fs::write(path.with_extension("pagedata"), "")?;
    for dir_ext in &["", "cache", "highlights", "thumbnails", "textconversion"]
    {
        if let Err(e) = fs::create_dir(path.with_extension(dir_ext))
            && e.kind() == ErrorKind::AlreadyExists
            && path.with_extension(dir_ext).is_dir()
        {
            Err(e)
                .wrap_err_with(|| format!("Failed to create {dir_ext} dir"))?;
        }
    }

    log::info!("added report on locked files (pdf)");
    Ok(())
}

pub fn remove_files() -> Result<()> {
    let path = Path::new(DIR).join(REPORT_UUID);
    assert!(!REPORT_UUID.is_empty(), "report uuid is empty str");
    let files = ["content", "metadata", "pagedata", "pdf"];
    let dirs = ["", "cache", "highlights", "thumbnails", "textconversion"];

    // check for the last dir we remove, if its not here neither will
    // the other files be so nothing can be removed
    if !path.with_extension(dirs.last().unwrap()).is_dir() {
        log::warn!("no weather report to remove");
        return Ok(());
    }

    for file_ext in &files {
        fs::remove_file(path.with_extension(file_ext))
            .wrap_err_with(|| format!("Failed to remove file: {file_ext}"))?;
    }
    for dir_ext in &dirs {
        fs::remove_dir_all(path.with_extension(dir_ext))
            .wrap_err_with(|| format!("Failed to remove dir: {dir_ext}"))?;
    }
    Ok(())
}

fn metadata() -> String {
    let unix_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!(
        "{{
    \"deleted\": false,
    \"lastModified\": \"{unix_ts}\",
    \"metadatamodified\": false,
    \"modified\": false,
    \"parent\": \"\",
    \"pinned\": false,
    \"synced\": true,
    \"type\": \"DocumentType\",
    \"version\": 1,
    \"visibleName\": \"Weather\"
}}"
    )
}

fn content(pages: usize) -> String {
    format!(
        "{{
    \"extraMetadata\": {{    
    }},
    \"fileType\": \"pdf\",
    \"fontName\": \"\",
    \"lastOpenedPage\": 0,
    \"lineHeight\": -1,
    \"margins\": 100,
    \"orientation\": \"portrait\",
    \"pageCount\": {pages},
    \"pages\": [
    ],
    \"textScale\": 1,
    \"transform\": {{
        \"m11\": 1,
        \"m12\": 0,
        \"m13\": 0,
        \"m21\": 0,
        \"m22\": 1,
        \"m23\": 0,
        \"m31\": 0,
        \"m32\": 0,
        \"m33\": 1
    }}
}}"
    )
}
