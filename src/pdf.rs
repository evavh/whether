use std::fs;

use printpdf::{
    FontId, Mm, Op, ParsedFont, PdfDocument, PdfPage, PdfSaveOptions, Point, Pt, Svg, TextItem, XObjectTransform
};

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

pub(crate) fn generate_pdf(weather: &str, image_path: &str, output_path: &str) {
    let mut warnings = Vec::new();
    let mut doc = PdfDocument::new("My first PDF");

    let svg = fs::read_to_string(image_path).unwrap();
    let image = Svg::parse(&svg, &mut warnings).unwrap();
    let noto = include_bytes!("../NotoSansMono-Regular.ttf");
    let font = ParsedFont::from_bytes(noto, 0, &mut warnings).unwrap();
    dbg!(&warnings);

    let image_id = doc.add_xobject(&image);
    let font_id = doc.add_font(&font);
    let text_pos = Point {
        x: Mm(10.0).into(),
        y: Mm(100.0).into(),
    }; // from bottom left

    let page1_contents = vec![
        // Op::UseXobject {
        //     id: image_id,
        //     transform: XObjectTransform {
        //         translate_x: Some(Mm(PLOT_POS_X_MM).into_pt()),
        //         translate_y: Some(Mm(PLOT_POS_Y_MM).into_pt()),
        //         rotate: None,
        //         scale_x: None,
        //         scale_y: None,
        //         dpi: Some(DPI),
        //     },
        // },
        Op::SetLineHeight { lh: Pt(33.0) },
        Op::SetWordSpacing { pt: Pt(33.0) },
        Op::SetCharacterSpacing { multiplier: 10.0 },
        Op::SetTextCursor { pos: text_pos },
        Op::WriteText {
            items: vec![TextItem::Text("hoi".to_owned())],
            font: font_id,
        },
    ];
    let page1 =
        PdfPage::new(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), page1_contents);

    let save_options = PdfSaveOptions {
        subset_fonts: true, // auto-subset fonts on save
        ..Default::default()
    };
    let pdf_bytes: Vec<u8> = doc
        .with_pages(vec![page1])
        .save(&save_options, &mut warnings);
    // dbg!(&warnings);
    fs::write(output_path, &pdf_bytes).unwrap();
}
