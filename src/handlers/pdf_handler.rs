use crate::{
    dtos::global::Response, services::pdf_service::PdfService, utils::error::ErrorResponse,
    AppState,
};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{
    body::BodyStream,
    error,
    http::header::{self, ContentDisposition, ContentType, HeaderMap},
    web::{self, Data},
    HttpResponse, Responder,
};
use futures::{StreamExt, TryStreamExt};
use genpdf::{elements, Scale};
use image::io::Reader as ImageReader;
use sanitize_filename::sanitize;
use serde_json::json;
use std::{
    borrow::Borrow,
    fs::File,
    io::{BufReader, BufWriter, Cursor, Read, Write},
    path::Path,
};

// Ex: https://git.sr.ht/~ireas/genpdf-rs/tree/master/examples/demo.rs
// Ex: https://github.com/tokio-rs/axum/discussions/608#discussioncomment-1789020
pub async fn get_genpdf_handler(data: Data<AppState>, mut payload: Multipart) -> impl Responder {
    let response_data = Response {
        status: "success",
        message: "Success".to_string(),
    };

    let pdf_service = PdfService::new(data.db.clone());

    // Load a font from the file system
    let font_family =
        genpdf::fonts::from_files("./fonts", "Poppins", None).expect("Failed to load font family");

    // Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);

    // Change the default settings
    doc.set_title("Demo document");

    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Add one or more elements
    doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
    doc.push(genpdf::elements::Paragraph::new(
        "This is a demo document 2.",
    ));

    doc.push(elements::Break::new(1.0));

    let mut table = elements::TableLayout::new(vec![2, 2]);
    let mut row = table.row();
    row.push_element(elements::Paragraph::new("Cell 1"));
    row.push_element(elements::Paragraph::new("Cell 2"));
    row.push().expect("Invalid table row");
    let mut row = table.row();
    row.push_element(elements::Paragraph::new("Cell 3"));
    row.push_element(elements::Paragraph::new("Cell 4"));
    row.push().expect("Invalid table row");

    doc.push(table);

    doc.push(elements::PageBreak::new());

    let mut image: Option<Vec<u8>> = None;
    let mut buffer = Vec::new();
    let mut file: Option<File> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(chunk) => chunk,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        status: "failed".to_string(),
                        message: e.to_string(),
                    });
                }
            };
            buffer.extend_from_slice(&data);
        }

        if field.name() == "file" {
            image = Some(buffer.clone());

            match field.content_disposition().get_filename() {
                Some(filename) => {
                    if let Some(extension) = filename.rfind(".") {
                        let extension = &filename[extension..];

                        let saved_name = "default.png";

                        let destination: String =
                            format!("{}{}", "storage/temp/", sanitize(filename));

                        let mut file = match File::create(destination.clone()) {
                            Ok(file) => file,
                            Err(e) => {
                                return HttpResponse::InternalServerError().json(ErrorResponse {
                                    status: "failed".to_string(),
                                    message: e.to_string(),
                                })
                            }
                        };

                        match file.write_all(&image.unwrap()) {
                            Ok(_) => {
                                let image = elements::Image::from_path(destination.clone())
                                    .expect("Failed to load test image")
                                    .with_alignment(genpdf::Alignment::Center) // Center the image on the page.
                                    .with_scale(Scale::new(0.5, 0.5));

                                doc.push(image);
                            }
                            Err(e) => {
                                return HttpResponse::InternalServerError().json(ErrorResponse {
                                    status: "failed".to_string(),
                                    message: e.to_string(),
                                })
                            }
                        }
                    };
                }
                None => (),
            };
        }
    }

    let pdf_path = "storage/pdf/lol.pdf";

    // Render the document and write it to a file
    let mut pdf_bytes: Vec<u8> = Vec::new();
    doc.render_to_file(&pdf_path);

    // &doc.render(&mut pdf_bytes).unwrap(); // TODO: handle error properly

    // Read the PDF file and send it as a response
    let mut file = std::fs::File::open(&pdf_path).unwrap();
    let mut buffer = Vec::new();
    std::io::copy(&mut file, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(ContentDisposition::attachment("genpdf.pdf"))
        .body(buffer)
}

// Ex: https://github.com/fschutt/printpdf
// pub async fn get_printpdf_handler(data: Data<AppState>) -> impl Responder {
//     let (doc, page1, layer1) =
//         PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
//     let current_layer = doc.get_page(page1).get_layer(layer1);

//     let font = doc
//         .add_external_font(File::open("fonts/Poppins-BlackItalic.ttf").unwrap())
//         .unwrap();
//     let font2 = doc
//         .add_external_font(File::open("fonts/Poppins-Regular.ttf").unwrap())
//         .unwrap();

//     current_layer.use_text(
//         "This pdf generated using printpdf",
//         12.0,
//         Mm(0.0),
//         Mm(200.0),
//         &font,
//     );

//     let text = "Lorem ipsum";
//     let text2 = "LOLL";

//     current_layer.begin_text_section();

//     // setup the general fonts.
//     // see the docs for these functions for details
//     current_layer.set_font(&font2, 33.0);
//     current_layer.set_text_cursor(Mm(10.0), Mm(184.0));
//     current_layer.set_line_height(33.0);
//     current_layer.set_word_spacing(3000.0);
//     current_layer.set_character_spacing(10.0);
//     current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);

//     // write two lines (one line break)
//     current_layer.write_text(text.clone(), &font2);
//     current_layer.add_line_break();
//     current_layer.write_text(text2.clone(), &font2);
//     current_layer.add_line_break();

//     // write one line, but write text2 in superscript
//     current_layer.write_text(text.clone(), &font2);
//     current_layer.set_line_offset(10.0);
//     current_layer.write_text(text2.clone(), &font2);

//     current_layer.end_text_section();

//     // // Quadratic shape. The "false" determines if the next (following)
//     // // point is a bezier handle (for curves)
//     // // If you want holes, simply reorder the winding of the points to be
//     // // counterclockwise instead of clockwise.
//     // let points1 = vec![
//     //     (Point::new(Mm(100.0), Mm(100.0)), false),
//     //     (Point::new(Mm(100.0), Mm(200.0)), false),
//     //     (Point::new(Mm(300.0), Mm(200.0)), false),
//     //     (Point::new(Mm(300.0), Mm(100.0)), false),
//     // ];

//     // // Is the shape stroked? Is the shape closed? Is the shape filled?
//     // let line1 = Line {
//     //     points: points1,
//     //     is_closed: true,
//     // };

//     // // Triangle shape
//     // // Note: Line is invisible by default, the previous method of
//     // // constructing a line is recommended!
//     // let mut line2 = Line::from_iter(vec![
//     //     (Point::new(Mm(150.0), Mm(150.0)), false),
//     //     (Point::new(Mm(150.0), Mm(250.0)), false),
//     //     (Point::new(Mm(350.0), Mm(250.0)), false),
//     // ]);

//     // // line2.set_stroke(true);
//     // line2.set_closed(false);
//     // // line2.set_fill(false);
//     // // line2.set_as_clipping_path(false);

//     // let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
//     // let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));
//     // let mut dash_pattern = LineDashPattern::default();
//     // dash_pattern.dash_1 = Some(20);

//     // current_layer.set_fill_color(fill_color);
//     // current_layer.set_outline_color(outline_color);
//     // current_layer.set_outline_thickness(10.0);

//     // // Draw first line
//     // current_layer.add_line(line1);

//     // let fill_color_2 = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
//     // let outline_color_2 = Color::Greyscale(Greyscale::new(0.45, None));

//     // // More advanced graphical options
//     // current_layer.set_overprint_stroke(true);
//     // current_layer.set_blend_mode(BlendMode::Seperable(SeperableBlendMode::Multiply));
//     // current_layer.set_line_dash_pattern(dash_pattern);
//     // current_layer.set_line_cap_style(LineCapStyle::Round);

//     // current_layer.set_fill_color(fill_color_2);
//     // current_layer.set_outline_color(outline_color_2);
//     // current_layer.set_outline_thickness(15.0);

//     // // draw second line
//     // current_layer.add_line(line2);

//     let mut pdf_bytes: Vec<u8> = doc.save_to_bytes().unwrap();
//     // doc.save(&mut BufWriter::new(
//     //     File::create("test_working.pdf").unwrap(),
//     // ))
//     // .unwrap();

//     HttpResponse::Ok()
//         .content_type(ContentType::html())
//         .insert_header(ContentDisposition::attachment("printpdf.pdf"))
//         .body(pdf_bytes)
// }
