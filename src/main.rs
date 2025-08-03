use std::{collections::BTreeMap, fs};

use hdf5_metno::types::FixedAscii;
use toml::Table;

mod files;

const CONFIG: &str = "config.toml";

fn main() {
    let config = fs::read_to_string(CONFIG).unwrap();
    let config: Table = config.parse().unwrap();
    let pixel_x: usize =
        config["pixel_x"].as_integer().unwrap().try_into().unwrap();
    let pixel_y: usize =
        config["pixel_y"].as_integer().unwrap().try_into().unwrap();

    let file = hdf5_metno::File::open("input/RAD_NL25_RAC_FM_202507201425.h5")
        .unwrap();

    let values: BTreeMap<_, _> = (1..=25)
        .into_iter()
        .map(|id| read_image(&file, id, pixel_x, pixel_y))
        .collect();

    dbg!(values);
}

fn read_image(
    file: &hdf5_metno::File,
    image_id: usize,
    pixel_x: usize,
    pixel_y: usize,
) -> (String, u16) {
    let image_group = file.group(&format!("image{image_id}")).unwrap();

    let image_time = image_group.attr("image_datetime_valid").unwrap();
    let image = image_group.dataset("image_data").unwrap();
    let image = image.read_2d::<u16>().unwrap();

    let value = image.get((pixel_x, pixel_y)).unwrap();
    let image_time = image_time
        .read_scalar::<FixedAscii<25>>()
        .unwrap()
        .as_str()
        .to_owned();
    (image_time, *value)
}
