use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Local, NaiveDateTime};
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use toml::Table;

const URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Serialize)]
struct Parameters {
    latitude: f64,
    longitude: f64,
    hourly: String,
    timezone: String,
    forecast_days: u8,
}

#[derive(Deserialize, Debug)]
struct Response {
    hourly: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    apparent_temperature: Vec<f64>,
    cloud_cover: Vec<usize>,
    precipitation: Vec<f64>,
    precipitation_probability: Vec<usize>,
    relative_humidity_2m: Vec<usize>,
    temperature_2m: Vec<f64>,
    time: Vec<String>,
    weather_code: Vec<usize>,
    wind_direction_10m: Vec<usize>,
    wind_gusts_10m: Vec<f64>,
    wind_speed_10m: Vec<f64>,
}

#[derive(Debug)]
pub(crate) struct Weather {
    apparent_temperature: f64,
    cloud_cover: usize,
    precipitation: f64,
    precipitation_probability: usize,
    relative_humidity_2m: usize,
    temperature_2m: f64,
    weather_code: usize,
    wind_direction_10m: usize,
    wind_gusts_10m: f64,
    wind_speed_10m: f64,
}

pub(crate) fn get_weather(
    config: &Table,
) -> BTreeMap<DateTime<Local>, Weather> {
    let lat = config.get("latitude").unwrap().as_float().unwrap();
    let lon = config.get("longitude").unwrap().as_float().unwrap();

    let data = get_data(lat, lon);
    let mut transposed = BTreeMap::new();
    for (i, time) in data.time.iter().enumerate() {
        let time =
            NaiveDateTime::parse_from_str(time.as_str(), "%Y-%m-%dT%H:%M")
                .unwrap()
                .and_local_timezone(Local)
                .unwrap();
        transposed.insert(
            time,
            Weather {
                apparent_temperature: data.apparent_temperature[i],
                cloud_cover: data.cloud_cover[i],
                precipitation: data.precipitation[i],
                precipitation_probability: data.precipitation_probability[i],
                relative_humidity_2m: data.relative_humidity_2m[i],
                temperature_2m: data.temperature_2m[i],
                weather_code: data.weather_code[i],
                wind_direction_10m: data.wind_direction_10m[i],
                wind_gusts_10m: data.wind_gusts_10m[i],
                wind_speed_10m: data.wind_speed_10m[i],
            },
        );
    }
    transposed
}

fn get_data(latitude: f64, longitude: f64) -> Data {
    let client = blocking::Client::new();
    let data_variables = String::from(
        "temperature_2m,relative_humidity_2m,apparent_temperature,cloud_cover,wind_speed_10m,wind_direction_10m,wind_gusts_10m,precipitation,precipitation_probability,weather_code",
    );
    let parameters = Parameters {
        latitude,
        longitude,
        hourly: data_variables,
        timezone: String::from("auto"),
        forecast_days: 1,
    };
    let response: Response = client
        .get(URL)
        .query(&parameters)
        .send()
        .unwrap()
        .json()
        .unwrap();
    response.hourly
}
