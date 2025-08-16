use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

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
    temperature_2m: f64,
    relative_humidity_2m: usize,
    apparent_temperature: f64,

    precipitation: f64,
    precipitation_probability: usize,

    cloud_cover: usize,
    // TODO: display with icon
    weather_code: usize,
    wind_direction_10m: usize,
    wind_gusts_10m: f64,
    wind_speed_10m: f64,
}

fn wind_direction(degrees: usize) -> String {
    match degrees as f64 {
        0.0..=11.25 => "N",
        11.25..=33.75 => "NNE",
        33.75..=56.25 => "NE",
        56.25..=78.75 => "ENE",
        78.75..=101.25 => "E",
        101.25..=123.75 => "ESE",
        123.75..=146.25 => "SE",
        146.25..=168.75 => "SSE",
        168.75..=191.25 => "S",
        191.25..=213.75 => "SSW",
        213.75..=236.25 => "SW",
        236.25..=258.75 => "WSW",
        258.75..=281.25 => "W",
        281.25..=303.75 => "WNW",
        303.75..=326.25 => "NW",
        326.25..=348.75 => "NNW",
        348.75..=360.0 => "N",
        _ => "INVALID",
    }
    .to_owned()
}

fn to_beaufort(wind_kph: f64) -> usize {
    if wind_kph < 1.0 {
        return 0;
    }
    match wind_kph.round() as usize {
        0..1 => 0,
        1..=5 => 1,
        6..=11 => 2,
        12..=19 => 3,
        20..=28 => 4,
        29..=38 => 5,
        39..=49 => 6,
        50..=61 => 7,
        62..=74 => 8,
        75..=88 => 9,
        89..=102 => 10,
        103..=117 => 11,
        118.. => 12,
    }
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp = format!(
            "🌡️ {}C (💧 {}%, 🤒 {}C)",
            self.temperature_2m,
            self.relative_humidity_2m,
            self.apparent_temperature
        );
        let precip = format!(
            "☔️ {} %, {} mm",
            self.precipitation_probability, self.precipitation
        );
        let clouds = format!("☁️ {}%", self.cloud_cover);
        let wind = format!(
            "{} {} ({} km/h)",
            wind_direction(self.wind_direction_10m),
            to_beaufort(self.wind_speed_10m),
            self.wind_gusts_10m
        );

        write!(f, "{temp}\t| {precip}\t| {clouds}\t| {wind}")
    }
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
