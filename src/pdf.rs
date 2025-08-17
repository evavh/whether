use std::{
    fs::{self, File},
    io::{BufWriter, Cursor},
};

use printpdf::{BuiltinFont, Mm, PdfDocument, Svg, SvgTransform};

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

const NOTO_BYTES: &[u8; 510612] = include_bytes!("../NotoSansMono-Regular.ttf");

pub(crate) fn generate_pdf(weather: &str, image_path: &str, output_path: &str) {
    let (doc, page1, layer1) = PdfDocument::new(
        "PDF_Document_title",
        Mm(PAGE_WIDTH_MM),
        Mm(PAGE_HEIGHT_MM),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let mut font_reader = Cursor::new(NOTO_BYTES.as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    let svg = fs::read_to_string(image_path).unwrap();
    let image = Svg::parse(&svg).unwrap().into_xobject(&current_layer);
    current_layer.set_font(&font, 10.0);
    image.add_to_layer(
        &current_layer,
        SvgTransform {
            translate_x: Some(Mm(PLOT_POS_X_MM).into_pt()),
            translate_y: Some(Mm(PLOT_POS_Y_MM).into_pt()),
            rotate: None,
            scale_x: None,
            scale_y: None,
            dpi: Some(DPI),
        },
    );

    let text = "Lorem ipsum";
    current_layer.begin_text_section();
    current_layer.set_text_cursor(Mm(10.0), Mm(100.0));
    current_layer.write_text(text, &font);
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(
        File::create("output/weather.pdf").unwrap(),
    ))
    .unwrap();
}
