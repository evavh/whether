use std::process::Command;

use crate::OUTPUT_DIR;

pub(crate) fn generate_pdf(output_path: &str) {
    Command::new("convert")
        .arg(format!("{OUTPUT_DIR}/plot.svg"))
        .arg(format!("{OUTPUT_DIR}/plot.png"))
        .output()
        .unwrap();
    Command::new("convert")
        .arg(format!("{OUTPUT_DIR}/table.svg"))
        .arg(format!("{OUTPUT_DIR}/table.png"))
        .output()
        .unwrap();
    Command::new("convert")
        .arg(format!("{OUTPUT_DIR}/plot.png"))
        .arg(format!("{OUTPUT_DIR}/table.png"))
        .arg(output_path)
        .output()
        .unwrap();
}
