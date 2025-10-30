use std::{fs, process::Command, thread, time::Duration};

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

    println!("Generating weather report...");
    let precip = precip::get_precip_data(&config);
    let weather = weather::get_weather(&config);

    draw_table("output/table.svg", weather);
    plot_precip("output/plot.svg", &precip);
    let output_path = format!("output/{REPORT_UUID}.pdf");
    generate_pdf(
        vec![
            "output/plot.svg".to_string(),
            "output/table.svg".to_string(),
        ],
        &output_path,
    );

    let to_copy = if args.first_run {
        files::create_metadata_files(1).unwrap();
        format!("output/{REPORT_UUID}*")
    } else {
        format!("output/{REPORT_UUID}.pdf")
    };

    println!("Start sending to remarkable");
    loop {
        let rsynced = Command::new("rsync")
            .arg(&to_copy)
            .arg("remarkable:/home/root/.local/share/remarkable/xochitl/")
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
