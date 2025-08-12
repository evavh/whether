use plotters::prelude::*;

use chrono::{DateTime, Local};

use std::collections::BTreeMap;

use crate::pdf::{PLOT_HEIGHT_PIXEL, PLOT_WIDTH_PIXEL};

const TEXT_SIZE_PT: u32 = 40;

pub(super) fn plot(path: &str, data: BTreeMap<DateTime<Local>, u16>) {
    let (&start_time, _) = data.first_key_value().unwrap();
    let (&end_time, _) = data.last_key_value().unwrap();
    let max_value = *data.values().max().unwrap() as f64;

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
            data.iter().map(|(t, y)| (*t, *y as f64)),
            &BLUE,
        ))
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'output' dir exists under current dir");
}
