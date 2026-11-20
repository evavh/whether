use std::{env, fs, process::Command, thread, time::Duration};

use clap::Parser;
use table::draw_table;
use toml::Table;

use files::REPORT_UUID;
use pdf::generate_pdf;
use plot::plot_precip;

mod files;
mod pdf;
mod plot;
mod precip;
mod table;
mod weather;

const CONFIG_FILE: &str = "whether.toml";
const OUTPUT_DIR: &str = "/tmp/whether_output";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    first_run: bool,
}

fn main() {
    let args = Args::parse();
    let config = fs::read_to_string(dbg!(format!(
        "{}/{}",
        env::var("XDG_CONFIG_HOME").unwrap(),
        CONFIG_FILE
    )))
    .unwrap();
    let config: Table = config.parse().unwrap();

    println!("Generating weather report...");
    let precip = precip::get_precip_data(&config);
    let weather = weather::get_weather(&config);

    let _ = fs::create_dir(OUTPUT_DIR); // Ignore file exists
    draw_table(&format!("{OUTPUT_DIR}/table.svg"), weather);
    plot_precip(&format!("{OUTPUT_DIR}/plot.svg"), &precip);
    let output_path = format!("{OUTPUT_DIR}/{REPORT_UUID}.pdf");
    generate_pdf(&output_path);

    let to_copy = if args.first_run {
        files::create_metadata_files(2).unwrap();
        vec![
            format!("{OUTPUT_DIR}/{REPORT_UUID}"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.cache"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.content"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.highlights"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.metadata"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.pagedata"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.pdf"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.textconversion"),
            format!("{OUTPUT_DIR}/{REPORT_UUID}.thumbnails"),
        ]
    } else {
        vec![format!("{OUTPUT_DIR}/{REPORT_UUID}.pdf")]
    };

    println!("Start sending to remarkable");
    for _ in 0..5 {
        // only retry 5 times
        let rsynced = dbg!(
            Command::new("rsync")
                .args(&to_copy)
                .arg("root@remarkable:/home/root/.local/share/remarkable/xochitl/")
        )
        .output()
        .unwrap();
        let stderr = str::from_utf8(&rsynced.stderr).unwrap();

        if stderr.contains("No route to host") {
            println!("Couldn't connect, retrying in 30s");
            thread::sleep(Duration::from_secs(30));
        } else if rsynced.status.success() {
            println!("Send succesful");
            return;
        } else {
            println!("Unexpected error: {}", stderr);
            return;
        }
    }
}
