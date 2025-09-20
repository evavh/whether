use std::fs::{self};

use printpdf::{
    Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Svg, XObjectTransform,
};

use crate::table::TABLE_HEIGHT_PIXEL;

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
const PLOT_POS_X_MM: f32 = MARGIN_MM;
const PLOT_POS_Y_MM: f32 = PAGE_HEIGHT_MM - PLOT_HEIGHT_MM - MARGIN_MM;
const TABLE_HEIGHT_MM: f32 = TABLE_HEIGHT_PIXEL / DPMM;

pub(crate) fn generate_pdf(image_paths: Vec<String>, output_path: &str) {
    let mut doc = PdfDocument::new("Weather report");
    let mut ops = Vec::new();

    add_image(&mut doc, &mut ops, &image_paths[0], PLOT_POS_Y_MM);
    add_image(
        &mut doc,
        &mut ops,
        &image_paths[1],
        PLOT_POS_Y_MM - TABLE_HEIGHT_MM,
    );

    let page1 = PdfPage::new(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), ops);

    let mut warnings = Vec::new();
    let pdf_bytes: Vec<u8> = doc
        .with_pages(vec![page1])
        .save(&PdfSaveOptions::default(), &mut warnings);
    fs::write(output_path, pdf_bytes).unwrap();
}

fn add_image(
    doc: &mut PdfDocument,
    ops: &mut Vec<Op>,
    image_path: &str,
    translate_y: f32,
) {
    let mut warnings = Vec::new();
    let svg = fs::read_to_string(image_path).unwrap();
    let image = Svg::parse(&svg, &mut warnings).unwrap();
    let image_id = doc.add_xobject(&image);

    ops.push(Op::UseXobject {
        id: image_id,
        transform: XObjectTransform {
            translate_x: Some(Mm(PLOT_POS_X_MM).into_pt()),
            translate_y: Some(Mm(translate_y).into_pt()),
            rotate: None,
            scale_x: None,
            scale_y: None,
            dpi: Some(DPI),
        },
    })
}
