use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use hdf5_metno::types::FixedAscii;
use reqwest::{Client, Method, header};
use serde::{Deserialize, Serialize};
use toml::Table;

mod files;

const CONFIG: &str = "config.toml";
const PRECIP_URL: &str = "https://api.dataplatform.knmi.nl/open-data/v1/datasets/radar_forecast/versions/2.0/files";

#[derive(Deserialize, Debug)]
struct PrecipResponse {
    files: Vec<PrecipFile>,
}

#[derive(Deserialize, Debug)]
struct PrecipFile {
    filename: String,
}

#[derive(Deserialize, Debug)]
struct FileUrl {
    temporaryDownloadUrl: String,
}

#[derive(Serialize)]
struct Params {
    maxKeys: usize,
    sorting: String,
    order_by: String,
}

#[derive(Clone, Copy)]
struct Pixel {
    x: usize,
    y: usize,
}

impl Pixel {
    fn from_config(config: &Table) -> Self {
        let x: usize =
            config["pixel_x"].as_integer().unwrap().try_into().unwrap();
        let y: usize =
            config["pixel_y"].as_integer().unwrap().try_into().unwrap();

        Self { x, y }
    }
}

#[tokio::main]
async fn main() {
    let config = fs::read_to_string(CONFIG).unwrap();
    let config: Table = config.parse().unwrap();
    let pixel = Pixel::from_config(&config);
    let knmi_api_key = config["knmi_api_key"].as_str().unwrap();

    let client = Client::new();
    let precip_file_list: PrecipResponse = client
        .get(PRECIP_URL)
        .header(header::AUTHORIZATION, knmi_api_key)
        .query(&Params {
            maxKeys: 1,
            sorting: String::from("desc"),
            order_by: String::from("lastModified"),
        })
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let latest_precip_filename = precip_file_list.files[0].filename.clone();
    let precip_file_url: FileUrl = client
        .get(&format!("{PRECIP_URL}/{latest_precip_filename}/url"))
        .header(header::AUTHORIZATION, knmi_api_key)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let precip_file_url = precip_file_url.temporaryDownloadUrl;

    let precip_path = &format!("input/{latest_precip_filename}");
    download_file(&precip_file_url, precip_path).await;

    let precip_file = hdf5_metno::File::open(precip_path).unwrap();

    let precip: BTreeMap<_, _> = (1..=25)
        .into_iter()
        .map(|id| read_image(&precip_file, id, pixel))
        .collect();

    dbg!(precip);
}

async fn download_file(url: &str, path: &str) {
    use tokio::io::AsyncWriteExt;

    let client = reqwest::Client::new();
    let mut response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let mut file = tokio::fs::File::create(path).await.unwrap();

    while let Some(mut item) = response.chunk().await.unwrap() {
        file.write_all_buf(&mut item).await.unwrap();
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
