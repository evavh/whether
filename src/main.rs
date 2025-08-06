use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::BufReader,
};

use grib::codetables::{CodeTable4_2, Lookup};
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

    let weather_file =
        File::open("input/HA43_N20_202508031200_00000_GB").unwrap();
    let weather_file = grib::from_reader(BufReader::new(weather_file)).unwrap();

    for (_index, submessage) in weather_file.iter() {
        let discipline = submessage.indicator().discipline;
        let category = submessage.prod_def().parameter_category().unwrap();
        let parameter = submessage.prod_def().parameter_number().unwrap();
        let parameter = CodeTable4_2::new(discipline, category)
            .lookup(usize::from(parameter));

        let forecast_time = submessage.prod_def().forecast_time().unwrap();

        let (first, _second) = submessage.prod_def().fixed_surfaces().unwrap();
        let elevation_level = first.value();

        println!(
            "{:<31} {:>14} {:>17}",
            parameter.to_string(),
            forecast_time.to_string(),
            elevation_level
        );
    }
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
