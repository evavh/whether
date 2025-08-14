use std::{fs, process::Command};

use clap::Parser;
use toml::Table;

use files::REPORT_UUID;
use pdf::generate_pdf;
use plot::plot;

mod files;
mod pdf;
mod plot;
mod precip;

const CONFIG: &str = "config.toml";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    first_run: bool,
}

fn main() {
    let args = Args::parse();
    let config = fs::read_to_string(CONFIG).unwrap();
    let config: Table = config.parse().unwrap();

    let precip = precip::get_precip_data(config);

    plot("output/plot.svg", &precip);
    let output_path = format!("output/{REPORT_UUID}.pdf");
    generate_pdf("output/plot.svg", &output_path);

    let to_copy = if args.first_run {
        files::create_metadata_files(1).unwrap();
        format!("output/{REPORT_UUID}*")
    } else {
        format!("output/{REPORT_UUID}.pdf")
    };
    // TODO: retry if failed because no connection
    let rsynced = Command::new("rsync")
        .arg(to_copy)
        .arg("remarkable:/home/root/.local/share/remarkable/xochitl/")
        .output()
        .unwrap();
    dbg!(rsynced);
}
