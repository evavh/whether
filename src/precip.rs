use std::{collections::BTreeMap, fs};

use chrono::{DateTime, Local, NaiveDateTime};
use hdf5_metno::types::FixedAscii;
use reqwest::{blocking, header};
use serde::{Deserialize, Serialize};
use toml::{Table, Value, map::Map};

const PRECIP_URL: &str = "https://api.dataplatform.knmi.nl/open-data/v1/datasets/radar_forecast/versions/2.0/files";

pub(super) fn get_precip_data(
    config: Map<String, Value>,
) -> BTreeMap<DateTime<Local>, u16> {
    let pixel = Pixel::from_config(&config);
    let knmi_api_key = config["knmi_api_key"].as_str().unwrap();
    let client = blocking::Client::new();

    let latest_filename = get_latest_filename(knmi_api_key, &client);
    let file_url = get_file_url(knmi_api_key, &client, &latest_filename);
    let path = &format!("input/{latest_filename}");

    download_file(client, file_url, path);
    let data = hdf5_metno::File::open(path).unwrap();

    (1..=25).map(|id| read_image(&data, id, pixel)).collect()
}

fn get_latest_filename(
    knmi_api_key: &str,
    client: &blocking::Client,
) -> String {
    let precip_file_list: ListResponse = client
        .get(PRECIP_URL)
        .header(header::AUTHORIZATION, knmi_api_key)
        .query(&ApiParams {
            maxKeys: 1,
            sorting: String::from("desc"),
            order_by: String::from("lastModified"),
        })
        .send()
        .unwrap()
        .json()
        .unwrap();
    let latest_precip_filename = precip_file_list.files[0].filename.clone();
    latest_precip_filename
}

fn get_file_url(
    knmi_api_key: &str,
    client: &blocking::Client,
    latest_filename: &String,
) -> String {
    let precip_file_url: FileUrl = client
        .get(format!("{PRECIP_URL}/{latest_filename}/url"))
        .header(header::AUTHORIZATION, knmi_api_key)
        .send()
        .unwrap()
        .json()
        .unwrap();
    let precip_file_url = precip_file_url.temporaryDownloadUrl;
    precip_file_url
}

fn download_file(client: blocking::Client, file_url: String, path: &String) {
    let file_content = client.get(file_url).send().unwrap().bytes().unwrap();
    fs::write(path, &file_content).unwrap();
}

fn read_image(
    file: &hdf5_metno::File,
    image_id: usize,
    pixel: Pixel,
) -> (DateTime<Local>, u16) {
    let image_group = file.group(&format!("image{image_id}")).unwrap();

    let image_time = image_group.attr("image_datetime_valid").unwrap();
    let image = image_group.dataset("image_data").unwrap();
    let image = image.read_2d::<u16>().unwrap();

    let value = image.get((pixel.x, pixel.y)).unwrap();
    let image_time = image_time.read_scalar::<FixedAscii<25>>().unwrap();
    let image_time = NaiveDateTime::parse_from_str(
        image_time.as_str(),
        "%d-%b-%Y;%H:%M:%S%.f",
    )
    .unwrap()
    .and_utc()
    .with_timezone(&Local);
    (image_time, *value)
}

#[derive(Deserialize, Debug)]
struct ListResponse {
    files: Vec<FileResponse>,
}

#[derive(Deserialize, Debug)]
struct FileResponse {
    filename: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct FileUrl {
    temporaryDownloadUrl: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct ApiParams {
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
