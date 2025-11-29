// Adapted from github.com/dvdsk/Book-safe/blob/main/src/report.rs,
// with permission

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use color_eyre::eyre::{Context, Result};

use crate::OUTPUT_DIR;

pub(crate) const REPORT_UUID: &str = "56f3ee9d-413b-4651-ac97-f9a9932a760a";

pub fn create_metadata_files(n_pages: usize) -> Result<()> {
    log::info!("report uuid: {REPORT_UUID} (constant)");
    if let Err(e) = fs::create_dir(OUTPUT_DIR)
        && e.kind() != ErrorKind::AlreadyExists
    {
        Err(e)
            .wrap_err_with(|| format!("Failed to create {OUTPUT_DIR} dir"))?;
    };
    let path = Path::new(OUTPUT_DIR).join(REPORT_UUID);

    fs::write(path.with_extension("content"), content(n_pages))?;
    fs::write(path.with_extension("metadata"), metadata())?;
    fs::write(path.with_extension("pagedata"), "")?;
    for dir_ext in &["cache", "highlights", "thumbnails", "textconversion"] {
        if let Err(e) = fs::create_dir(path.with_extension(dir_ext))
            && e.kind() != ErrorKind::AlreadyExists
        {
            Err(e)
                .wrap_err_with(|| format!("Failed to create {dir_ext} dir"))?;
        }
    }

    log::info!("added report on locked files (pdf)");
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
