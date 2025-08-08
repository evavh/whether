use std::{collections::BTreeMap, fs};

use hdf5_metno::types::FixedAscii;
use toml::Table;

mod files;

const CONFIG: &str = "config.toml";

#[derive(Clone, Copy)]
struct Pixel {
    x: usize,
    y: usize,
}

impl Pixel {
    fn from_config(config: Table) -> Self {
        let x: usize =
            config["pixel_x"].as_integer().unwrap().try_into().unwrap();
        let y: usize =
            config["pixel_y"].as_integer().unwrap().try_into().unwrap();

        Self { x, y }
    }
}

fn main() {
    let config = fs::read_to_string(CONFIG).unwrap();
    let config: Table = config.parse().unwrap();
    let pixel = Pixel::from_config(config);

    let precip_file =
        hdf5_metno::File::open("input/RAD_NL25_RAC_FM_202507201425.h5")
            .unwrap();

    let precip: BTreeMap<_, _> = (1..=25)
        .into_iter()
        .map(|id| read_image(&precip_file, id, pixel))
        .collect();

    dbg!(precip);
}

fn read_image(
    file: &hdf5_metno::File,
    image_id: usize,
    pixel: Pixel,
) -> (String, u16) {
    let image_group = file.group(&format!("image{image_id}")).unwrap();

    let image_time = image_group.attr("image_datetime_valid").unwrap();
    let image = image_group.dataset("image_data").unwrap();
    let image = image.read_2d::<u16>().unwrap();

    let value = image.get((pixel.x, pixel.y)).unwrap();
    let image_time = image_time
        .read_scalar::<FixedAscii<25>>()
        .unwrap()
        .as_str()
        .to_owned();
    (image_time, *value)
}
