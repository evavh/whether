use std::collections::BTreeMap;
use std::fs;

use chrono::{DateTime, Local};

use crate::plot::PLOT_WIDTH_PIXEL;
use crate::weather::{Weather, to_beaufort, wind_direction};

const ROW_HEIGHT_PIXEL: usize = 30;
pub(crate) const TABLE_HEIGHT_PIXEL: f32 = ROW_HEIGHT_PIXEL as f32 * 26.;

pub(crate) fn draw_table(path: &str, data: BTreeMap<DateTime<Local>, Weather>) {
    assert_eq!(data.len(), 24);
    let mut svg = format!(
        r##"
<svg width="{PLOT_WIDTH_PIXEL}" height="{TABLE_HEIGHT_PIXEL}" viewBox="0 0 {PLOT_WIDTH_PIXEL} {TABLE_HEIGHT_PIXEL}" xmlns="http://www.w3.org/2000/svg">
<rect x="0" y="0" width="{PLOT_WIDTH_PIXEL}" height="{TABLE_HEIGHT_PIXEL}" opacity="1" fill="#FFFFFF" stroke="none"/>
<g xmlns="http://www.w3.org/2000/svg" id="rowGroup">
<text x="{ROW_HEIGHT_PIXEL}" y="30" font-size="18px" text-anchor="start">
         <tspan x="30" font-weight="bold" fill="crimson" text-anchor="start">Time</tspan>
         <tspan x="90">T</tspan>
         <tspan x="120">Feel</tspan>
         <tspan x="150">Hum</tspan>
         <tspan x="190">Clouds</tspan>
         <tspan x="240">Wind</tspan>
      </text>
"##
    );
    for (row, (time, weather)) in data.into_iter().enumerate() {
        svg.push_str(&render_row(row, time, weather));
    }
    svg.push_str("</g></svg>");

    fs::write(path, svg).unwrap();
}

fn render_row(row: usize, time: DateTime<Local>, weather: Weather) -> String {
    format!(
        r##"<text x="{ROW_HEIGHT_PIXEL}" y="{}" font-size="18px" text-anchor="start">
         <tspan x="30" font-weight="bold" fill="crimson" text-anchor="start">{}</tspan>
         <tspan x="90">{:.0}</tspan>
         <tspan x="120">{:.0}</tspan>
         <tspan x="150">{}%</tspan>
         <tspan x="190">{}%</tspan>
         <tspan x="240">{} {}</tspan>
      </text>"##,
        (row + 2) * ROW_HEIGHT_PIXEL,
        time.format("%H:%M"),
        weather.temperature_2m,
        weather.apparent_temperature,
        weather.relative_humidity_2m,
        weather.cloud_cover,
        wind_direction(weather.wind_direction_10m),
        to_beaufort(weather.wind_speed_10m),
    )
}
