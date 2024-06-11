use std::borrow::Borrow;
use std::fs::{remove_file, File};
use std::io::{BufWriter, Write};

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use genpdf::elements::Paragraph;
use genpdf::style::{Effect, Style};
use genpdf::{elements, style, Scale};
use genpdf::{Document, Size};
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::*;
use sanitize_filename::sanitize;
use sqlx::MySqlPool;
use uuid::Uuid;

struct Item {
    no: i32,
    keterangan: String,
    kode_ac: String,
    deskripsi: String,
    nilai: i32,
}

#[derive(Debug)]
pub struct PdfService {
    pool: MySqlPool,
}

impl PdfService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn generate_pdf_service(
        &self,
        mut payload: Multipart,
        filename: String,
    ) -> Result<Vec<u8>, String> {
        let height = 140.0;
        let width = 240.0;
        let margin = 10.1;
        let color_primary = Color::Cmyk(Cmyk::new(0.79, 0.09, 1.0, 0.01, None));
        let color_white = Color::Rgb(Rgb::new(2.55, 2.55, 2.55, None));
        let color_black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

        let (doc, page1, layer1) =
            PdfDocument::new("Demo document", Mm(width), Mm(height), "Page 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);
        let ikt_layer = doc.get_page(page1).get_layer(layer1);
        let codein_layer = doc.get_page(page1).get_layer(layer1);

        let font_bold = doc
            .add_external_font(File::open("assets/fonts/poppins/Poppins-Bold.ttf").unwrap())
            .unwrap();
        let font_medium = doc
            .add_external_font(File::open("assets/fonts/poppins/Poppins-Medium.ttf").unwrap())
            .unwrap();
        let font_regular = doc
            .add_external_font(File::open("assets/fonts/poppins/Poppins-Regular.ttf").unwrap())
            .unwrap();

        let title = "BUKTI PENERIMAAN BANK BCA (2264100550)";
        current_layer.use_text(
            title,
            14.0,
            Mm(0.0 + margin),
            Mm(height - (margin + 3.0)),
            &font_bold,
        );

        let mut ikt_file = File::open("assets/images/Logo IKT.jpg").unwrap();
        let ikt_img =
            Image::try_from(image_crate::codecs::jpeg::JpegDecoder::new(&mut ikt_file).unwrap())
                .unwrap();
        ikt_img.add_to_layer(
            ikt_layer,
            ImageTransform {
                translate_x: Some(Mm(width - (margin + 70.0))),
                translate_y: Some(Mm(height - (margin + 10.0))),
                scale_x: Some(0.2),
                scale_y: Some(0.2),
                ..Default::default()
            },
        );

        let mut codein_file = File::open("assets/images/Powered by Codein.jpg").unwrap();
        let codein_img =
            Image::try_from(image_crate::codecs::jpeg::JpegDecoder::new(&mut codein_file).unwrap())
                .unwrap();
        codein_img.add_to_layer(
            codein_layer,
            ImageTransform {
                translate_x: Some(Mm(0.0 + (margin - 1.0))),
                translate_y: Some(Mm(height - (margin + 12.0))),
                scale_x: Some(0.08),
                scale_y: Some(0.08),
                ..Default::default()
            },
        );

        // Line shape
        let mut line = Line::from_iter(vec![
            (Point::new(Mm(-3.0), Mm(height - (margin + 16.0))), false),
            (Point::new(Mm(125.0), Mm(height - (margin + 16.0))), false),
        ]);
        current_layer.set_line_cap_style(LineCapStyle::Round);
        current_layer.set_outline_color(color_primary.clone());
        current_layer.set_outline_thickness(8.0);
        current_layer.add_line(line);

        let mut body_y = 16.0;
        let mut gap = 4.5;

        let no_voucher = format!("Nomor Voucher      : {}", "B011.2024.01.0181");
        current_layer.use_text(
            no_voucher,
            8.0,
            Mm(152.0 + margin),
            Mm(height - (margin + body_y)),
            &font_regular,
        );

        body_y += gap;

        let tanggal = format!("Tanggal                    : {}", "31 / 01 / 2024");
        current_layer.use_text(
            tanggal,
            8.0,
            Mm(152.0 + margin),
            Mm(height - (margin + body_y)),
            &font_regular,
        );

        body_y += gap + 2.0;

        let divisi = format!(
            "Divisi                                                             : {}",
            "Finance (Debitur - Kasir - Kas Kol)"
        );
        current_layer.use_text(
            divisi,
            8.0,
            Mm(0.0 + margin),
            Mm(height - (margin + body_y)),
            &font_regular,
        );

        body_y += gap;

        let count_print = format!(
            "Count Print                                                  : {} kali",
            2
        );
        current_layer.use_text(
            count_print,
            8.0,
            Mm(0.0 + margin),
            Mm(height - (margin + body_y)),
            &font_regular,
        );

        body_y += gap;

        let amount_receive = format!(
            "Dengan ini diterima dana sebesar       : Rp {}",
            "54.470.000"
        );
        current_layer.use_text(
            amount_receive,
            8.0,
            Mm(0.0 + margin),
            Mm(height - (margin + body_y)),
            &font_regular,
        );

        body_y += gap;

        let description = format!(
            "Keterangan                                                 : {}",
            "spr0050"
        );
        current_layer.use_text(
            description,
            8.0,
            Mm(0.0 + margin),
            Mm(height - (margin + body_y)),
            &font_regular,
        );

        body_y += gap;

        let mut trow_y = body_y + 10.0;
        let trow_height = 6.0;

        let mut thead_point = vec![
            (
                Point::new(Mm(margin), Mm(height - (trow_y + trow_height))),
                false,
            ),
            (Point::new(Mm(margin), Mm(height - trow_y)), false),
            (Point::new(Mm(width - margin), Mm(height - trow_y)), false),
            (
                Point::new(Mm(width - margin), Mm(height - (trow_y + trow_height))),
                false,
            ),
        ];

        let thead_line = Polygon {
            rings: vec![thead_point],
            mode: PaintMode::Fill,
            winding_order: WindingOrder::NonZero,
        };

        current_layer.set_fill_color(color_primary.clone());
        current_layer.add_polygon(thead_line);

        current_layer.set_fill_color(color_white.clone());

        let mut row_y = Mm(height - (trow_y + trow_height / 2.0 + 1.0));

        let row1_x = Mm(margin + (trow_height / 2.0) + 0.0);
        current_layer.use_text("No.", 8.0, row1_x, row_y, &font_medium);

        let row2_x = Mm(margin + (trow_height / 2.0) + 12.0);
        current_layer.use_text("Keterangan", 8.0, row2_x, row_y, &font_medium);

        let row3_x = Mm(margin + (trow_height / 2.0) + 60.0);
        current_layer.use_text("Kode A/C", 8.0, row3_x, row_y, &font_medium);

        let row4_x = Mm(margin + (trow_height / 2.0) + 100.0);
        current_layer.use_text("Deskripsi", 8.0, row4_x, row_y, &font_medium);

        let row5_x = Mm(margin + (trow_height / 2.0) + 180.0);
        current_layer.use_text("Nilai", 8.0, row5_x, row_y, &font_medium);

        let dummies: Vec<Item> = vec![
            Item {
                no: 1,
                nilai: 57000000,
                keterangan: "spr0050".to_string(),
                deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
                kode_ac: "1.1.201.100".to_string(),
            },
            Item {
                no: 2,
                nilai: 300000000,
                keterangan: "2311-274".to_string(),
                deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
                kode_ac: "2.2.201.200".to_string(),
            },
        ];

        current_layer.set_fill_color(color_black.clone());
        row_y -= Mm(1.3);

        for dummy in dummies {
            row_y -= Mm(trow_height);

            current_layer.use_text(dummy.no.to_string(), 8.0, row1_x, row_y, &font_medium);

            current_layer.use_text(dummy.keterangan, 8.0, row2_x, row_y, &font_medium);

            current_layer.use_text(dummy.kode_ac, 8.0, row3_x, row_y, &font_medium);

            current_layer.use_text(dummy.deskripsi, 8.0, row4_x, row_y, &font_medium);

            current_layer.use_text(
                format!("Rp {}", dummy.nilai),
                8.0,
                row5_x,
                row_y,
                &font_medium,
            );
        }

        let mut note_y = row_y - (Mm(trow_height + 3.0));

        current_layer.use_text(
            "Diterima dengan :",
            8.0,
            Mm(0.0 + margin),
            note_y,
            &font_regular,
        );

        current_layer.use_text("Total Nilai :", 8.0, row5_x - Mm(27.0), note_y, &font_bold);
        current_layer.use_text("Rp 57470250", 8.0, row5_x, note_y, &font_bold);

        let mut amount_spelled =
            "Lima Puluh Tujuh Juta Empat Ratus Tujuh Puluh Ribu Dua Ratus Lima Puluh";
        let words: Vec<&str> = amount_spelled.split(' ').collect();

        let mut spelled_point = vec![
            (
                Point::new(
                    Mm(margin + 50.0),
                    note_y - Mm(5.0 * (1.0 + words.len() as f32 / 10.0) + 5.0),
                ),
                false,
            ),
            (Point::new(Mm(margin + 50.0), note_y + Mm(1.0)), false),
            (Point::new(Mm(margin + 140.0), note_y + Mm(1.0)), false),
            (
                Point::new(
                    Mm(margin + 140.0),
                    note_y - Mm(5.0 * (1.0 + words.len() as f32 / 10.0) + 5.0),
                ),
                false,
            ),
        ];

        let spelled_line = Polygon {
            rings: vec![spelled_point],
            mode: PaintMode::Stroke,
            winding_order: WindingOrder::NonZero,
        };

        current_layer.set_fill_color(color_white.clone());
        current_layer.set_outline_color(color_primary.clone());
        current_layer.set_outline_thickness(2.0);
        current_layer.add_polygon(spelled_line);

        current_layer.set_fill_color(color_black.clone());

        current_layer.use_text(
            "Terbilang,",
            8.0,
            Mm(margin + 53.0),
            note_y - Mm(4.0),
            &font_regular,
        );

        let mut i = 1.0;

        for word in words.chunks(10) {
            current_layer.use_text(
                word.join(" "),
                8.0,
                Mm(margin + 53.0),
                note_y - Mm((4.0 + (4.0 * i))),
                &font_regular,
            );

            i += 1.0;
        }

        let mut giro_point = vec![
            (Point::new(Mm(margin), note_y - Mm(11.0)), false),
            (Point::new(Mm(margin), note_y - Mm(4.0)), false),
            (Point::new(Mm(margin + 16.0), note_y - Mm(4.0)), false),
            (Point::new(Mm(margin + 16.0), note_y - Mm(11.0)), false),
        ];

        let giro_line = Polygon {
            rings: vec![giro_point],
            mode: PaintMode::Stroke,
            winding_order: WindingOrder::NonZero,
        };

        current_layer.set_outline_color(color_primary.clone());
        current_layer.set_outline_thickness(2.0);
        current_layer.add_polygon(giro_line);

        current_layer.use_text(
            "Giro Bilyet",
            8.0,
            Mm(20.0 + margin),
            note_y - Mm(8.0),
            &font_regular,
        );

        let mut footer_y = (note_y - Mm(5.0 * (1.0 + words.len() as f32 / 10.0) + 5.0)) - Mm(10.0);

        let footer1_x = Mm(85.0 + margin);
        current_layer.use_text("Admin AR,", 8.0, footer1_x, footer_y, &font_regular);

        let footer2_x = Mm(120.0 + margin);
        current_layer.use_text("SPV Finance,", 8.0, footer2_x, footer_y, &font_regular);

        let footer3_x = Mm(155.0 + margin);
        current_layer.use_text("Manager Finance", 8.0, footer3_x, footer_y, &font_regular);
        current_layer.use_text(
            "Accounting & Tax,",
            8.0,
            footer3_x,
            footer_y - Mm(3.5),
            &font_regular,
        );

        let footer4_x = Mm(200.0 + margin);
        current_layer.use_text("Kasir,", 8.0, footer4_x, footer_y, &font_regular);

        footer_y = footer_y - Mm(18.0);
        current_layer.use_text(
            "(                                     )",
            8.0,
            footer1_x - Mm(8.0),
            footer_y,
            &font_regular,
        );

        current_layer.use_text(
            "(                                     )",
            8.0,
            footer2_x - Mm(5.0),
            footer_y,
            &font_regular,
        );

        current_layer.use_text(
            "(                                     )",
            8.0,
            footer3_x - Mm(2.0),
            footer_y,
            &font_regular,
        );

        current_layer.use_text(
            "(                                     )",
            8.0,
            footer4_x - Mm(11.0),
            footer_y,
            &font_regular,
        );

        let pdf_name = format!("{}", Uuid::new_v4());
        let pdf_path = format!("storage/pdf/{}.pdf", pdf_name);

        // Render the document and write it to a file
        doc.save(&mut BufWriter::new(File::create(&pdf_path).unwrap()))
            .unwrap();

        // Read the PDF file and send it as a response
        let mut file = std::fs::File::open(&pdf_path).unwrap();
        let mut buffer = Vec::new();
        std::io::copy(&mut file, &mut buffer).unwrap();

        Ok(buffer)
    }

    // pub async fn generate_pdf_service(
    //     &self,
    //     mut payload: Multipart,
    //     filename: String,
    // ) -> Result<Vec<u8>, String> {
    //     // Load a font from the file system
    //     let font_family = genpdf::fonts::from_files("./fonts", "Poppins", None)
    //         .expect("Failed to load font family");

    //     // Create a document and set the default font family
    //     let mut doc = genpdf::Document::new(font_family);

    //     // ? PRS
    //     doc.set_paper_size(Size::new(240, 140));

    //     // Change the default settings
    //     doc.set_title("Demo document");

    //     // Customize the pages
    //     let mut decorator = genpdf::SimplePageDecorator::new();
    //     decorator.set_margins(10);
    //     doc.set_page_decorator(decorator);

    //     let style = style::Style::default();

    //     let title_style = style::StyledStr::new("bold", Effect::Bold);

    //     let mut header_table = elements::TableLayout::new(vec![1, 2]);
    //     let mut row = header_table.row();

    //     let mut title = Paragraph::default();
    //     title.push_styled(
    //         "BUKTI PENERIMAAN BANK BCA (2264100550)",
    //         style::Effect::Bold,
    //     );
    //     row.push_element(title);
    //     // doc.push(title);

    //     let ikt_logo = elements::Image::from_path("assets/Logo IKT.jpg")
    //         .expect("Failed to load test image")
    //         .with_alignment(genpdf::Alignment::Right) // Center the image on the page.
    //         .with_scale(genpdf::Scale::new(0.1, 0.1));
    //     row.push_element(ikt_logo);
    //     // doc.push(ikt_logo);

    //     row.push().expect("Invalid table row");
    //     doc.push(header_table);

    //     let codein_logo = elements::Image::from_path("assets/Powered by Codein.jpg")
    //         .expect("Failed to load test image")
    //         .with_alignment(genpdf::Alignment::Left) // Center the image on the page.
    //         .with_scale(genpdf::Scale::new(0.1, 0.1));
    //     let logo = elements::FramedElement::new(codein_logo);
    //     doc.push(logo);

    // doc.push(elements::Break::new(1.0));

    // let mut table = elements::TableLayout::new(vec![2, 2]);
    // let mut row = table.row();
    // row.push_element(elements::Paragraph::new("Cell 1"));
    // row.push_element(elements::Paragraph::new("Cell 2"));
    // row.push().expect("Invalid table row");
    // let mut row = table.row();
    // row.push_element(elements::Paragraph::new("Cell 3"));
    // row.push_element(elements::Paragraph::new("Cell 4"));
    // row.push().expect("Invalid table row");

    // doc.push(table);
    // doc.push(elements::PageBreak::new());

    // ? Uploded Image
    // while let Ok(Some(mut field)) = payload.try_next().await {
    //     let mut buffer = Vec::new();

    //     while let Some(chunk) = field.next().await {
    //         let data = match chunk {
    //             Ok(chunk) => chunk,
    //             Err(e) => {
    //                 return Err(e.to_string());
    //             }
    //         };
    //         buffer.extend_from_slice(&data);
    //     }

    //     if field.name() == "file" {
    //         match field.content_disposition().get_filename() {
    //             Some(filename) => {
    //                 if let Some(extension) = filename.rfind(".") {
    //                     let extension = &filename[extension..];

    //                     let saved_name = format!("{}.png", Uuid::new_v4());

    //                     let destination: String =
    //                         format!("{}{}", "storage/temp/", sanitize(saved_name));
    //                     println!("{}", destination);

    //                     let mut file = match File::create(destination.clone()) {
    //                         Ok(file) => file,
    //                         Err(e) => {
    //                             return Err(e.to_string());
    //                         }
    //                     };

    //                     match file.write_all(&buffer) {
    //                         Ok(_) => {
    //                             let image = elements::Image::from_path(destination.clone())
    //                                 .expect("Failed to load test image")
    //                                 .with_alignment(genpdf::Alignment::Right) // Center the image on the page.
    //                                 .with_scale(Scale::new(0.5, 0.5));

    //                             // row.push_element(image);

    //                             remove_file(destination);
    //                         }
    //                         Err(e) => {
    //                             return Err(e.to_string());
    //                         }
    //                     }
    //                 };
    //             }
    //             None => (),
    //         };
    //     }
    // }

    //     let pdf_name = format!("{}", Uuid::new_v4());
    //     let pdf_path = format!("storage/pdf/{}.pdf", pdf_name);

    //     // Render the document and write it to a file
    //     let mut pdf_bytes: Vec<u8> = Vec::new();
    //     let _ = doc.render_to_file(&pdf_path);

    //     // Read the PDF file and send it as a response
    //     let mut file = std::fs::File::open(&pdf_path).unwrap();
    //     let mut buffer = Vec::new();
    //     std::io::copy(&mut file, &mut buffer).unwrap();

    //     Ok(buffer)
    // }
}
