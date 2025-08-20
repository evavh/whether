use std::collections::BTreeMap;
use std::fs;

use chrono::{DateTime, Local};

use crate::pdf::PLOT_WIDTH_PIXEL;
use crate::weather::Weather;

const ROW_HEIGHT_PIXEL: usize = 30;

pub(crate) fn draw_table(path: &str, data: BTreeMap<DateTime<Local>, Weather>) {
    let table_height_pixel = ROW_HEIGHT_PIXEL * (data.len() + 2);
    let mut svg = format!(
        r##"
<svg width="{PLOT_WIDTH_PIXEL}" height="{table_height_pixel}" viewBox="0 0 {PLOT_WIDTH_PIXEL} {table_height_pixel}" xmlns="http://www.w3.org/2000/svg">
<rect x="0" y="0" width="{PLOT_WIDTH_PIXEL}" height="{table_height_pixel}" opacity="1" fill="#FFFFFF" stroke="none"/>
<g xmlns="http://www.w3.org/2000/svg" id="rowGroup">
<text x="{ROW_HEIGHT_PIXEL}" y="30" font-size="18px" text-anchor="start">
         <tspan x="30" font-weight="bold" fill="crimson" text-anchor="start">Time</tspan>
         <tspan x="90">T</tspan>
         <tspan x="120">Feel</tspan>
         <tspan x="150">Hum</tspan>
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
      </text>"##,
        (row + 2) * ROW_HEIGHT_PIXEL,
        time.format("%H:%M"),
        weather.temperature_2m,
        weather.apparent_temperature,
        weather.relative_humidity_2m
    )
}
