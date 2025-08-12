use std::fs;

use printpdf::{
    Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Svg, XObjectTransform,
};

pub(crate) fn generate_pdf(image_path: &str, output_path: &str) {
    let mut warnings = Vec::new();
    let mut doc = PdfDocument::new("My first PDF");

    let svg = fs::read_to_string(image_path).unwrap();
    let image = Svg::parse(&svg, &mut warnings).unwrap();
    dbg!(&warnings);

    let id = doc.add_xobject(&image);

    let page1_contents = vec![Op::UseXobject {
        id,
        transform: XObjectTransform::default(),
    }];
    let page1 = PdfPage::new(Mm(157.2), Mm(209.6), page1_contents);

    let pdf_bytes: Vec<u8> = doc
        .with_pages(vec![page1])
        .save(&PdfSaveOptions::default(), &mut warnings);
    dbg!(&warnings);
    fs::write(output_path, &pdf_bytes).unwrap();
}
