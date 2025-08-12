use std::fs;

use pdf::generate_pdf;
use toml::Table;

mod files;
mod pdf;
mod plot;
mod precip;

const CONFIG: &str = "config.toml";

fn main() {
    let config = fs::read_to_string(CONFIG).unwrap();
    let config: Table = config.parse().unwrap();

    let precip = precip::get_precip_data(config);

    plot::plot("output/plot.svg", precip);

    generate_pdf("output/plot.svg", "output/report.pdf");

    // TODO: put pdf on remarkable
}
