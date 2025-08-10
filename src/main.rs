use std::fs;

use toml::Table;

mod files;
mod precip;

const CONFIG: &str = "config.toml";

fn main() {
    let config = fs::read_to_string(CONFIG).unwrap();
    let config: Table = config.parse().unwrap();

    let precip = precip::get_precip_data(config);

    dbg!(precip);
}

