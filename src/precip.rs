use std::{collections::BTreeMap, fs};

use hdf5_metno::types::FixedAscii;
use reqwest::header;
use serde::{Deserialize, Serialize};
use toml::Table;

const PRECIP_URL: &str = "https://api.dataplatform.knmi.nl/open-data/v1/datasets/radar_forecast/versions/2.0/files";

#[derive(Deserialize, Debug)]
struct PrecipResponse {
    files: Vec<PrecipFile>,
}

#[derive(Deserialize, Debug)]
struct PrecipFile {
    filename: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct FileUrl {
    temporaryDownloadUrl: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Params {
    maxKeys: usize,
    sorting: String,
    order_by: String,
}

#[derive(Clone, Copy)]
struct PrecipPixel {
    x: usize,
    y: usize,
}

impl PrecipPixel {
    fn from_config(config: &Table) -> Self {
        let x: usize =
            config["pixel_x"].as_integer().unwrap().try_into().unwrap();
        let y: usize =
            config["pixel_y"].as_integer().unwrap().try_into().unwrap();

        Self { x, y }
    }
}

pub(super) fn get_precip_data(
    config: toml::map::Map<String, toml::Value>,
) -> BTreeMap<String, u16> {
    let pixel = PrecipPixel::from_config(&config);
    let knmi_api_key = config["knmi_api_key"].as_str().unwrap();

    let client = reqwest::blocking::Client::new();
    let precip_file_list: PrecipResponse = client
        .get(PRECIP_URL)
        .header(header::AUTHORIZATION, knmi_api_key)
        .query(&Params {
            maxKeys: 1,
            sorting: String::from("desc"),
            order_by: String::from("lastModified"),
        })
        .send()
        .unwrap()
        .json()
        .unwrap();
    let latest_precip_filename = precip_file_list.files[0].filename.clone();
    let precip_file_url: FileUrl = client
        .get(format!("{PRECIP_URL}/{latest_precip_filename}/url"))
        .header(header::AUTHORIZATION, knmi_api_key)
        .send()
        .unwrap()
        .json()
        .unwrap();
    let precip_file_url = precip_file_url.temporaryDownloadUrl;

    let precip_path = &format!("input/{latest_precip_filename}");

    let file_content =
        client.get(precip_file_url).send().unwrap().bytes().unwrap();
    fs::write(precip_path, &file_content).unwrap();

    let precip_file = hdf5_metno::File::open(precip_path).unwrap();

    (1..=25)
        .map(|id| read_image(&precip_file, id, pixel))
        .collect()
}

fn read_image(
    file: &hdf5_metno::File,
    image_id: usize,
    pixel: PrecipPixel,
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
