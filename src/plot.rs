use plotters::prelude::*;

use chrono::{DateTime, Local};

use std::collections::BTreeMap;

const DPI: f32 = 300.0;
const PAGE_WIDTH_MM: f32 = 157.2;
const PAGE_HEIGHT_MM: f32 = 209.6;
const MARGIN_MM: f32 = 15.0;
const PLOT_ASPECT_RATIO: f32 = 3.0;

const DPMM: f32 = DPI / 25.4;
const PLOT_WIDTH_MM: f32 = PAGE_WIDTH_MM - MARGIN_MM * 2.;
const PLOT_HEIGHT_MM: f32 = PLOT_WIDTH_MM / PLOT_ASPECT_RATIO;
pub(crate) const PLOT_WIDTH_PIXEL: f32 = PLOT_WIDTH_MM * DPMM;
pub(crate) const PLOT_HEIGHT_PIXEL: f32 = PLOT_HEIGHT_MM * DPMM;

const TEXT_SIZE_PT: u32 = 40;

pub(super) fn plot_precip(path: &str, data: &BTreeMap<DateTime<Local>, u16>) {
    let (&start_time, _) = data.first_key_value().unwrap();
    let (&end_time, _) = data.last_key_value().unwrap();
    let max_value = f64::from(*data.values().max().unwrap());

    let root = SVGBackend::new(
        path,
        (
            PLOT_WIDTH_PIXEL.round() as u32,
            PLOT_HEIGHT_PIXEL.round() as u32,
        ),
    )
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Precipitation forecast", ("sans-serif", TEXT_SIZE_PT))
        .set_label_area_size(LabelAreaPosition::Left, 3 * TEXT_SIZE_PT)
        .set_label_area_size(LabelAreaPosition::Bottom, TEXT_SIZE_PT)
        .build_cartesian_2d(start_time..end_time, 0.0..max_value.max(10.0))
        .unwrap();

    chart
        .configure_mesh()
        .x_label_formatter(&|t: &DateTime<Local>| t.format("%H:%M").to_string())
        .x_label_style(("sans-serif", TEXT_SIZE_PT))
        .y_label_style(("sans-serif", TEXT_SIZE_PT))
        .max_light_lines(4)
        .y_desc("Precipitation [mm]")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            data.iter().map(|(t, y)| (*t, f64::from(*y))),
            &BLUE,
        ))
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'output' dir exists under current dir");
}
