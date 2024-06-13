use actix_multipart::Multipart;
use actix_web::http::header;
use futures::{StreamExt, TryStreamExt};
use genpdf::elements::{Break, FrameCellDecorator, Paragraph, Text};
use genpdf::style::{Effect, Style};
use genpdf::{elements, style, Alignment, Element, Margins, Mm, Position, Scale};
use genpdf::{Document, Size};
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::*;
use sanitize_filename::sanitize;
use sqlx::MySqlPool;
use std::borrow::Borrow;
use std::fs::{self, remove_file, File};
use std::io::{BufWriter, Write};
use std::os::windows;
use std::process::Command;
use std::rc::Rc;
use typst::diag::EcoString;
use typst::eval::Tracer;
use typst::foundations::Smart;
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::World;
use uuid::Uuid;

use crate::utils::typst_wrapper_world::TypstWrapperWorld;

#[derive(Debug, Clone)]
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

    pub async fn generate_typst_service(&self) -> () {
        let content = r#"
#import "@preview/polylux:0.3.1": *
#import themes.simple: *

#set page(paper: "presentation-16-9")

#show: simple-theme.with()

#title-slide[
= Hello, World!
A document (+ `polylux` library) rendered with `Typst`!
]"#
        .to_owned();
        // Create world with content.
        let world = TypstWrapperWorld::new("./".to_owned(), content);

        // Render document
        let mut tracer = Tracer::default();
        let document = typst::compile(&world, &mut tracer).expect("Error compiling typst.");

        // Output to pdf and svg
        let pdf = typst_pdf::pdf(&document, Smart::Auto, None);
        fs::write("./output.pdf", pdf).expect("Error writing PDF.");
        println!("Created pdf: `./output.pdf`");

        ()
    }

    pub async fn generate_pdf_service(
        &self,
        mut payload: Multipart,
        filename: String,
    ) -> Result<Vec<u8>, String> {
        // ? Data
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
            Item {
                no: 3,
                nilai: 57000000,
                keterangan: "spr0050".to_string(),
                deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
                kode_ac: "1.1.201.100".to_string(),
            },
            Item {
                no: 4,
                nilai: 300000000,
                keterangan: "2311-274".to_string(),
                deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
                kode_ac: "2.2.201.200".to_string(),
            },
            Item {
                no: 5,
                nilai: 57000000,
                keterangan: "spr0050".to_string(),
                deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
                kode_ac: "1.1.201.100".to_string(),
            },
            // Item {
            //     no: 6,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
            // Item {
            //     no: 7,
            //     nilai: 57000000,
            //     keterangan: "spr0050".to_string(),
            //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "1.1.201.100".to_string(),
            // },
            // Item {
            //     no: 8,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
            // Item {
            //     no: 9,
            //     nilai: 57000000,
            //     keterangan: "spr0050".to_string(),
            //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "1.1.201.100".to_string(),
            // },
            // Item {
            //     no: 10,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
            // Item {
            //     no: 11,
            //     nilai: 57000000,
            //     keterangan: "spr0050".to_string(),
            //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "1.1.201.100".to_string(),
            // },
            // Item {
            //     no: 12,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
            // Item {
            //     no: 13,
            //     nilai: 57000000,
            //     keterangan: "spr0050".to_string(),
            //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "1.1.201.100".to_string(),
            // },
            // Item {
            //     no: 14,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
            // Item {
            //     no: 15,
            //     nilai: 57000000,
            //     keterangan: "spr0050".to_string(),
            //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "1.1.201.100".to_string(),
            // },
            // Item {
            //     no: 16,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
            // Item {
            //     no: 17,
            //     nilai: 57000000,
            //     keterangan: "spr0050".to_string(),
            //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "1.1.201.100".to_string(),
            // },
            // Item {
            //     no: 18,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
            // Item {
            //     no: 19,
            //     nilai: 57000000,
            //     keterangan: "spr0050".to_string(),
            //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "1.1.201.100".to_string(),
            // },
            // Item {
            //     no: 20,
            //     nilai: 300000000,
            //     keterangan: "2311-274".to_string(),
            //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
            //     kode_ac: "2.2.201.200".to_string(),
            // },
        ];

        // Load a font from the file system
        let font_family = genpdf::fonts::from_files("./assets/fonts/poppins", "Poppins", None)
            .expect("Failed to load font family");

        // Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family);

        let style = style::Style::default();

        let white_color = style::Color::Rgb(255, 255, 255);
        let primary_color = style::Color::Rgb(46, 164, 73);

        // ? PRS
        doc.set_paper_size(Size::new(240, 140));

        // Change the default settings
        doc.set_title("Demo document");

        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(Margins::trbl(5, 10, 10, 10));
        decorator.set_header(move |page| {
            let ikt_logo = elements::Image::from_path("assets/images/Logo IKT.jpg")
                .expect("Failed to load test image")
                .with_position(Position::new(-1, -5))
                .with_scale(genpdf::Scale::new(0.2, 0.2));

            let codein_logo = elements::Image::from_path("assets/images/Powered by Codein.jpg")
                .expect("Failed to load test image")
                .with_position(Position::new(-1, 1))
                .with_scale(genpdf::Scale::new(0.1, 0.1));

            let divider = elements::Image::from_path("assets/images/Divider.jpg")
                .expect("Failed to load test image")
                .with_position(Position::new(-15, -12))
                .with_scale(genpdf::Scale::new(1, 1.2));

            let bg_table_header = elements::Image::from_path("assets/images/Table header.jpg")
                .expect("Failed to load test image")
                .with_position(Position::new(0, 0))
                .with_scale(genpdf::Scale::new(1.21, 1.1));

            let mut table_title = elements::TableLayout::new(vec![2, 1]);
            table_title.set_cell_decorator(FrameCellDecorator::new(false, false, false));
            let mut row_title = table_title.row();
            row_title.push_element(
                Paragraph::new("BUKTI PENERIMAAN BANK BCA (2264100550)")
                    .styled(Style::new().bold().with_font_size(14)),
            );
            row_title.push_element(ikt_logo);
            row_title.push();

            let mut table_detail = elements::TableLayout::new(vec![2, 1]);
            table_detail.set_cell_decorator(FrameCellDecorator::new(false, false, false));
            let mut row_detail = table_detail.row();
            row_detail.push_element(elements::Paragraph::new(""));
            row_detail.push_element(
                Paragraph::new(format!("Nomor Voucher      : {}", "B011.2024.01.0181"))
                    .styled(Style::new().with_font_size(8)),
            );
            row_detail.push();

            let mut row_detail = table_detail.row();
            row_detail.push_element(elements::Paragraph::new(""));
            row_detail.push_element(
                elements::Paragraph::new(format!(
                    "Tanggal                    : {}",
                    "31 / 01 / 2024"
                ))
                .styled(Style::new().with_font_size(8)),
            );
            row_detail.push();

            let mut table_body = elements::TableLayout::new(vec![1, 1, 1, 1, 1]);
            table_body.set_cell_decorator(FrameCellDecorator::new(false, false, false));
            let mut row_body = table_body.row();
            row_body.push_element(
                elements::Paragraph::new("No.").styled(Style::new().with_color(white_color)),
            );
            row_body.push_element(
                elements::Paragraph::new("Keterangan").styled(Style::new().with_color(white_color)),
            );
            row_body.push_element(
                elements::Paragraph::new("Kode A/C").styled(Style::new().with_color(white_color)),
            );
            row_body.push_element(
                elements::Paragraph::new("Deskripsi").styled(Style::new().with_color(white_color)),
            );
            row_body.push_element(
                elements::Paragraph::new("Nilai").styled(Style::new().with_color(white_color)),
            );
            row_body.push();

            let mut layout = elements::LinearLayout::vertical()
                .element(
                    elements::Paragraph::new(format!("Page {}", page))
                        .aligned(Alignment::Left)
                        .styled(Style::new().with_font_size(8)),
                )
                .element(table_title)
                .element(codein_logo)
                .element(Break::new(1.8))
                .element(table_detail)
                .element(divider)
                .element(
                    Paragraph::new(format!(
                        "Divisi                                                             : {}",
                        "Finance (Debitur - Kasir - Kas Kol)"
                    ))
                    .styled(Style::new().with_font_size(8)),
                )
                .element(
                    Paragraph::new(format!(
                        "Count Print                                                  : {} kali",
                        2
                    ))
                    .styled(Style::new().with_font_size(8)),
                )
                .element(
                    Paragraph::new(format!(
                        "Dengan ini diterima dana sebesar       : Rp {}",
                        "54.470.000"
                    ))
                    .styled(Style::new().with_font_size(8)),
                )
                .element(
                    Paragraph::new(format!(
                        "Keterangan                                                 : {}",
                        "spr0050"
                    ))
                    .styled(Style::new().with_font_size(8)),
                )
                .element(Break::new(1))
                .element(bg_table_header)
                .element(table_body)
                .element(Break::new(0.5));

            layout
        });
        doc.set_page_decorator(decorator);

        let mut table = elements::TableLayout::new(vec![1, 1, 1, 1, 1]);
        table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push_element(elements::Paragraph::new("Cell 2"));
        row.push_element(elements::Paragraph::new("Cell 1"));
        row.push().expect("Invalid table row");

        let mut row = table.row();
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push_element(elements::Paragraph::new("Cell 4"));
        row.push_element(elements::Paragraph::new("Cell 3"));
        row.push().expect("Invalid table row");

        doc.push(table);

        // ? Uploded Image
        while let Ok(Some(mut field)) = payload.try_next().await {
            let mut buffer = Vec::new();

            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };
                buffer.extend_from_slice(&data);
            }

            if field.name() == "file" {
                match field.content_disposition().get_filename() {
                    Some(filename) => {
                        if let Some(extension) = filename.rfind(".") {
                            let extension = &filename[extension..];

                            let saved_name = format!("{}.png", Uuid::new_v4());

                            let destination: String =
                                format!("{}{}", "storage/temp/", sanitize(saved_name));
                            println!("{}", destination);

                            let mut file = match File::create(destination.clone()) {
                                Ok(file) => file,
                                Err(e) => {
                                    return Err(e.to_string());
                                }
                            };

                            match file.write_all(&buffer) {
                                Ok(_) => {
                                    let image = elements::Image::from_path(destination.clone())
                                        .expect("Failed to load test image")
                                        .with_alignment(genpdf::Alignment::Right) // Center the image on the page.
                                        .with_scale(Scale::new(0.5, 0.5));

                                    // row.push_element(image);

                                    remove_file(destination);
                                }
                                Err(e) => {
                                    return Err(e.to_string());
                                }
                            }
                        };
                    }
                    None => (),
                };
            }
        }

        let pdf_name = format!("{}", Uuid::new_v4());
        let pdf_path = format!("storage/pdf/{}.pdf", pdf_name);

        // Render the document and write it to a file
        let mut pdf_bytes: Vec<u8> = Vec::new();
        let _ = doc.render_to_file(&pdf_path);

        // Read the PDF file and send it as a response
        let mut file = std::fs::File::open(&pdf_path).unwrap();
        let mut buffer = Vec::new();
        std::io::copy(&mut file, &mut buffer).unwrap();

        Ok(buffer)
    }

    // pub async fn generate_pdf_service1(
    //     &self,
    //     mut payload: Multipart,
    //     filename: String,
    // ) -> Result<Vec<u8>, String> {
    //     let height = 140.0;
    //     let width = 240.0;
    //     let margin = 10.1;
    //     let color_primary = Color::Cmyk(Cmyk::new(0.79, 0.09, 1.0, 0.01, None));
    //     let color_white = Color::Rgb(Rgb::new(2.55, 2.55, 2.55, None));
    //     let color_black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    //     // ? Data
    //     let dummies: Vec<Item> = vec![
    //         Item {
    //             no: 1,
    //             nilai: 57000000,
    //             keterangan: "spr0050".to_string(),
    //             deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //             kode_ac: "1.1.201.100".to_string(),
    //         },
    //         Item {
    //             no: 2,
    //             nilai: 300000000,
    //             keterangan: "2311-274".to_string(),
    //             deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //             kode_ac: "2.2.201.200".to_string(),
    //         },
    //         Item {
    //             no: 3,
    //             nilai: 57000000,
    //             keterangan: "spr0050".to_string(),
    //             deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //             kode_ac: "1.1.201.100".to_string(),
    //         },
    //         Item {
    //             no: 4,
    //             nilai: 300000000,
    //             keterangan: "2311-274".to_string(),
    //             deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //             kode_ac: "2.2.201.200".to_string(),
    //         },
    //         Item {
    //             no: 5,
    //             nilai: 57000000,
    //             keterangan: "spr0050".to_string(),
    //             deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //             kode_ac: "1.1.201.100".to_string(),
    //         },
    //         // Item {
    //         //     no: 6,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //         // Item {
    //         //     no: 7,
    //         //     nilai: 57000000,
    //         //     keterangan: "spr0050".to_string(),
    //         //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "1.1.201.100".to_string(),
    //         // },
    //         // Item {
    //         //     no: 8,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //         // Item {
    //         //     no: 9,
    //         //     nilai: 57000000,
    //         //     keterangan: "spr0050".to_string(),
    //         //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "1.1.201.100".to_string(),
    //         // },
    //         // Item {
    //         //     no: 10,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //         // Item {
    //         //     no: 11,
    //         //     nilai: 57000000,
    //         //     keterangan: "spr0050".to_string(),
    //         //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "1.1.201.100".to_string(),
    //         // },
    //         // Item {
    //         //     no: 12,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //         // Item {
    //         //     no: 13,
    //         //     nilai: 57000000,
    //         //     keterangan: "spr0050".to_string(),
    //         //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "1.1.201.100".to_string(),
    //         // },
    //         // Item {
    //         //     no: 14,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //         // Item {
    //         //     no: 15,
    //         //     nilai: 57000000,
    //         //     keterangan: "spr0050".to_string(),
    //         //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "1.1.201.100".to_string(),
    //         // },
    //         // Item {
    //         //     no: 16,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //         // Item {
    //         //     no: 17,
    //         //     nilai: 57000000,
    //         //     keterangan: "spr0050".to_string(),
    //         //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "1.1.201.100".to_string(),
    //         // },
    //         // Item {
    //         //     no: 18,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //         // Item {
    //         //     no: 19,
    //         //     nilai: 57000000,
    //         //     keterangan: "spr0050".to_string(),
    //         //     deskripsi: "Piutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "1.1.201.100".to_string(),
    //         // },
    //         // Item {
    //         //     no: 20,
    //         //     nilai: 300000000,
    //         //     keterangan: "2311-274".to_string(),
    //         //     deskripsi: "Hutang Usaha - Pihak Berelasi".to_string(),
    //         //     kode_ac: "2.2.201.200".to_string(),
    //         // },
    //     ];
    //     let (doc, page1, layer1) =
    //         PdfDocument::new("Demo document", Mm(width), Mm(height), "Page 1");
    //     let current_layer = doc.get_page(page1).get_layer(layer1);
    //     let font_bold = doc
    //         .add_external_font(File::open("assets/fonts/poppins/Poppins-Bold.ttf").unwrap())
    //         .unwrap();
    //     let font_medium = doc
    //         .add_external_font(File::open("assets/fonts/poppins/Poppins-Medium.ttf").unwrap())
    //         .unwrap();
    //     let font_regular = doc
    //         .add_external_font(File::open("assets/fonts/poppins/Poppins-Regular.ttf").unwrap())
    //         .unwrap();
    //     // ? header sampai keterangan
    //     let body_y = PdfService::generate_pdf_header(
    //         &doc,
    //         page1.clone(),
    //         layer1.clone(),
    //         current_layer.clone(),
    //         height.clone(),
    //         width.clone(),
    //         margin.clone(),
    //         color_primary.clone(),
    //         color_white.clone(),
    //         color_black.clone(),
    //         font_bold.clone(),
    //         font_medium.clone(),
    //         font_regular.clone(),
    //     )
    //     .await;
    //     // ? table
    //     let (row_y, trow_height, row5_x) = PdfService::generate_pdf_table(
    //         &doc,
    //         page1.clone(),
    //         layer1.clone(),
    //         current_layer.clone(),
    //         height.clone(),
    //         width.clone(),
    //         margin.clone(),
    //         color_primary.clone(),
    //         color_white.clone(),
    //         color_black.clone(),
    //         font_bold.clone(),
    //         font_medium.clone(),
    //         font_regular.clone(),
    //         body_y.clone(),
    //         dummies.clone(),
    //     )
    //     .await;
    //     // ? diterima dengan sampai TTD
    //     PdfService::generate_pdf_footer(
    //         &doc,
    //         page1.clone(),
    //         layer1.clone(),
    //         current_layer.clone(),
    //         height.clone(),
    //         width.clone(),
    //         margin.clone(),
    //         color_primary.clone(),
    //         color_white.clone(),
    //         color_black.clone(),
    //         font_bold.clone(),
    //         font_medium.clone(),
    //         font_regular.clone(),
    //         row_y.clone(),
    //         trow_height.clone(),
    //         row5_x.clone(),
    //     )
    //     .await;
    //     let pdf_name = format!("{}", Uuid::new_v4());
    //     let pdf_path = format!("storage/pdf/{}.pdf", pdf_name);
    //     // Render the document and write it to a file
    //     doc.save(&mut BufWriter::new(File::create(&pdf_path).unwrap()))
    //         .unwrap();

    //     // Read the PDF file and send it as a response
    //     let mut file = std::fs::File::open(&pdf_path).unwrap();
    //     let mut buffer = Vec::new();
    //     std::io::copy(&mut file, &mut buffer).unwrap();

    //     Ok(buffer)
    // }

    // pub async fn generate_pdf_header(
    //     doc: &PdfDocumentReference,
    //     page: PdfPageIndex,
    //     layer: PdfLayerIndex,
    //     current_layer: PdfLayerReference,
    //     height: f32,
    //     width: f32,
    //     margin: f32,
    //     color_primary: Color,
    //     color_white: Color,
    //     color_black: Color,
    //     font_bold: IndirectFontRef,
    //     font_medium: IndirectFontRef,
    //     font_regular: IndirectFontRef,
    // ) -> f32 {
    //     let ikt_layer = doc.get_page(page).get_layer(layer);
    //     let codein_layer = doc.get_page(page).get_layer(layer);

    //     let title = "BUKTI PENERIMAAN BANK BCA (2264100550)";
    //     current_layer.use_text(
    //         title,
    //         14.0,
    //         Mm(0.0 + margin),
    //         Mm(height - (margin + 3.0)),
    //         &font_bold,
    //     );

    //     let mut ikt_file = File::open("assets/images/Logo IKT.jpg").unwrap();
    //     let ikt_img =
    //         Image::try_from(image_crate::codecs::jpeg::JpegDecoder::new(&mut ikt_file).unwrap())
    //             .unwrap();
    //     ikt_img.add_to_layer(
    //         ikt_layer,
    //         ImageTransform {
    //             translate_x: Some(Mm(width - (margin + 70.0))),
    //             translate_y: Some(Mm(height - (margin + 10.0))),
    //             scale_x: Some(0.2),
    //             scale_y: Some(0.2),
    //             ..Default::default()
    //         },
    //     );

    //     let mut codein_file = File::open("assets/images/Powered by Codein.jpg").unwrap();
    //     let codein_img =
    //         Image::try_from(image_crate::codecs::jpeg::JpegDecoder::new(&mut codein_file).unwrap())
    //             .unwrap();
    //     codein_img.add_to_layer(
    //         codein_layer,
    //         ImageTransform {
    //             translate_x: Some(Mm(0.0 + (margin - 1.0))),
    //             translate_y: Some(Mm(height - (margin + 12.0))),
    //             scale_x: Some(0.08),
    //             scale_y: Some(0.08),
    //             ..Default::default()
    //         },
    //     );

    //     // Line shape
    //     let mut line = Line::from_iter(vec![
    //         (Point::new(Mm(-3.0), Mm(height - (margin + 16.0))), false),
    //         (Point::new(Mm(125.0), Mm(height - (margin + 16.0))), false),
    //     ]);
    //     current_layer.set_line_cap_style(LineCapStyle::Round);
    //     current_layer.set_outline_color(color_primary.clone());
    //     current_layer.set_outline_thickness(8.0);
    //     current_layer.add_line(line);

    //     let mut body_y = 16.0;
    //     let mut gap = 4.5;

    //     let no_voucher = format!("Nomor Voucher      : {}", "B011.2024.01.0181");
    //     current_layer.use_text(
    //         no_voucher,
    //         8.0,
    //         Mm(152.0 + margin),
    //         Mm(height - (margin + body_y)),
    //         &font_regular,
    //     );

    //     body_y += gap;

    //     let tanggal = format!("Tanggal                    : {}", "31 / 01 / 2024");
    //     current_layer.use_text(
    //         tanggal,
    //         8.0,
    //         Mm(152.0 + margin),
    //         Mm(height - (margin + body_y)),
    //         &font_regular,
    //     );

    //     body_y += gap + 2.0;

    //     let divisi = format!(
    //         "Divisi                                                             : {}",
    //         "Finance (Debitur - Kasir - Kas Kol)"
    //     );
    //     current_layer.use_text(
    //         divisi,
    //         8.0,
    //         Mm(0.0 + margin),
    //         Mm(height - (margin + body_y)),
    //         &font_regular,
    //     );

    //     body_y += gap;

    //     let count_print = format!(
    //         "Count Print                                                  : {} kali",
    //         2
    //     );
    //     current_layer.use_text(
    //         count_print,
    //         8.0,
    //         Mm(0.0 + margin),
    //         Mm(height - (margin + body_y)),
    //         &font_regular,
    //     );

    //     body_y += gap;

    //     let amount_receive = format!(
    //         "Dengan ini diterima dana sebesar       : Rp {}",
    //         "54.470.000"
    //     );
    //     current_layer.use_text(
    //         amount_receive,
    //         8.0,
    //         Mm(0.0 + margin),
    //         Mm(height - (margin + body_y)),
    //         &font_regular,
    //     );

    //     body_y += gap;

    //     let description = format!(
    //         "Keterangan                                                 : {}",
    //         "spr0050"
    //     );
    //     current_layer.use_text(
    //         description,
    //         8.0,
    //         Mm(0.0 + margin),
    //         Mm(height - (margin + body_y)),
    //         &font_regular,
    //     );

    //     body_y += gap;

    //     body_y
    // }

    // pub async fn generate_pdf_table(
    //     doc: &PdfDocumentReference,
    //     page: PdfPageIndex,
    //     layer: PdfLayerIndex,
    //     current_layer: PdfLayerReference,
    //     height: f32,
    //     width: f32,
    //     margin: f32,
    //     color_primary: Color,
    //     color_white: Color,
    //     color_black: Color,
    //     font_bold: IndirectFontRef,
    //     font_medium: IndirectFontRef,
    //     font_regular: IndirectFontRef,
    //     body_y: f32,
    //     dummies: Vec<Item>,
    // ) -> (Mm, f32, Mm) {
    //     let mut trow_y = body_y + 10.0;
    //     let trow_height = 6.0;

    //     let mut thead_point = vec![
    //         (
    //             Point::new(Mm(margin), Mm(height - (trow_y + trow_height))),
    //             false,
    //         ),
    //         (Point::new(Mm(margin), Mm(height - trow_y)), false),
    //         (Point::new(Mm(width - margin), Mm(height - trow_y)), false),
    //         (
    //             Point::new(Mm(width - margin), Mm(height - (trow_y + trow_height))),
    //             false,
    //         ),
    //     ];

    //     let thead_line = Polygon {
    //         rings: vec![thead_point],
    //         mode: PaintMode::Fill,
    //         winding_order: WindingOrder::NonZero,
    //     };

    //     current_layer.set_fill_color(color_primary.clone());
    //     current_layer.add_polygon(thead_line);

    //     current_layer.set_fill_color(color_white.clone());

    //     let mut row_y = Mm(height - (trow_y + trow_height / 2.0 + 1.0));

    //     let row1_x = Mm(margin + (trow_height / 2.0) + 0.0);
    //     current_layer.use_text("No.", 8.0, row1_x, row_y, &font_medium);

    //     let row2_x = Mm(margin + (trow_height / 2.0) + 12.0);
    //     current_layer.use_text("Keterangan", 8.0, row2_x, row_y, &font_medium);

    //     let row3_x = Mm(margin + (trow_height / 2.0) + 60.0);
    //     current_layer.use_text("Kode A/C", 8.0, row3_x, row_y, &font_medium);

    //     let row4_x = Mm(margin + (trow_height / 2.0) + 100.0);
    //     current_layer.use_text("Deskripsi", 8.0, row4_x, row_y, &font_medium);

    //     let row5_x = Mm(margin + (trow_height / 2.0) + 180.0);
    //     current_layer.use_text("Nilai", 8.0, row5_x, row_y, &font_medium);

    //     current_layer.set_fill_color(color_black.clone());
    //     row_y -= Mm(1.3);

    //     for dummy in dummies {
    //         row_y -= Mm(trow_height);

    //         current_layer.use_text(dummy.no.to_string(), 8.0, row1_x, row_y, &font_medium);

    //         current_layer.use_text(dummy.keterangan, 8.0, row2_x, row_y, &font_medium);

    //         current_layer.use_text(dummy.kode_ac, 8.0, row3_x, row_y, &font_medium);

    //         current_layer.use_text(dummy.deskripsi, 8.0, row4_x, row_y, &font_medium);

    //         current_layer.use_text(
    //             format!("Rp {}", dummy.nilai),
    //             8.0,
    //             row5_x,
    //             row_y,
    //             &font_medium,
    //         );
    //     }

    //     (row_y, trow_height, row5_x)
    // }

    // pub async fn generate_pdf_footer(
    //     doc: &PdfDocumentReference,
    //     page: PdfPageIndex,
    //     layer: PdfLayerIndex,
    //     current_layer: PdfLayerReference,
    //     height: f32,
    //     width: f32,
    //     margin: f32,
    //     color_primary: Color,
    //     color_white: Color,
    //     color_black: Color,
    //     font_bold: IndirectFontRef,
    //     font_medium: IndirectFontRef,
    //     font_regular: IndirectFontRef,
    //     row_y: Mm,
    //     trow_height: f32,
    //     row5_x: Mm,
    // ) -> () {
    //     let mut note_y = row_y - (Mm(trow_height + 3.0));

    //     current_layer.use_text(
    //         "Diterima dengan :",
    //         8.0,
    //         Mm(0.0 + margin),
    //         note_y,
    //         &font_regular,
    //     );

    //     current_layer.use_text("Total Nilai :", 8.0, row5_x - Mm(27.0), note_y, &font_bold);
    //     current_layer.use_text("Rp 57470250", 8.0, row5_x, note_y, &font_bold);

    //     let mut amount_spelled =
    //         "Lima Puluh Tujuh Juta Empat Ratus Tujuh Puluh Ribu Dua Ratus Lima Puluh";
    //     let words: Vec<&str> = amount_spelled.split(' ').collect();

    //     let mut spelled_point = vec![
    //         (
    //             Point::new(
    //                 Mm(margin + 50.0),
    //                 note_y - Mm(5.0 * (1.0 + words.len() as f32 / 10.0) + 5.0),
    //             ),
    //             false,
    //         ),
    //         (Point::new(Mm(margin + 50.0), note_y + Mm(1.0)), false),
    //         (Point::new(Mm(margin + 140.0), note_y + Mm(1.0)), false),
    //         (
    //             Point::new(
    //                 Mm(margin + 140.0),
    //                 note_y - Mm(5.0 * (1.0 + words.len() as f32 / 10.0) + 5.0),
    //             ),
    //             false,
    //         ),
    //     ];

    //     let spelled_line = Polygon {
    //         rings: vec![spelled_point],
    //         mode: PaintMode::Stroke,
    //         winding_order: WindingOrder::NonZero,
    //     };

    //     current_layer.set_fill_color(color_white.clone());
    //     current_layer.set_outline_color(color_primary.clone());
    //     current_layer.set_outline_thickness(2.0);
    //     current_layer.add_polygon(spelled_line);

    //     current_layer.set_fill_color(color_black.clone());

    //     current_layer.use_text(
    //         "Terbilang,",
    //         8.0,
    //         Mm(margin + 53.0),
    //         note_y - Mm(4.0),
    //         &font_regular,
    //     );

    //     let mut i = 1.0;

    //     for word in words.chunks(10) {
    //         current_layer.use_text(
    //             word.join(" "),
    //             8.0,
    //             Mm(margin + 53.0),
    //             note_y - Mm(4.0 + (4.0 * i)),
    //             &font_regular,
    //         );

    //         i += 1.0;
    //     }

    //     let mut giro_point = vec![
    //         (Point::new(Mm(margin), note_y - Mm(11.0)), false),
    //         (Point::new(Mm(margin), note_y - Mm(4.0)), false),
    //         (Point::new(Mm(margin + 16.0), note_y - Mm(4.0)), false),
    //         (Point::new(Mm(margin + 16.0), note_y - Mm(11.0)), false),
    //     ];

    //     let giro_line = Polygon {
    //         rings: vec![giro_point],
    //         mode: PaintMode::Stroke,
    //         winding_order: WindingOrder::NonZero,
    //     };

    //     current_layer.set_outline_color(color_primary.clone());
    //     current_layer.set_outline_thickness(2.0);
    //     current_layer.add_polygon(giro_line);

    //     current_layer.use_text(
    //         "Giro Bilyet",
    //         8.0,
    //         Mm(20.0 + margin),
    //         note_y - Mm(8.0),
    //         &font_regular,
    //     );

    //     let mut footer_y = (note_y - Mm(5.0 * (1.0 + words.len() as f32 / 10.0) + 5.0)) - Mm(10.0);

    //     let footer1_x = Mm(85.0 + margin);
    //     current_layer.use_text("Admin AR,", 8.0, footer1_x, footer_y, &font_regular);

    //     let footer2_x = Mm(120.0 + margin);
    //     current_layer.use_text("SPV Finance,", 8.0, footer2_x, footer_y, &font_regular);

    //     let footer3_x = Mm(155.0 + margin);
    //     current_layer.use_text("Manager Finance", 8.0, footer3_x, footer_y, &font_regular);
    //     current_layer.use_text(
    //         "Accounting & Tax,",
    //         8.0,
    //         footer3_x,
    //         footer_y - Mm(3.5),
    //         &font_regular,
    //     );

    //     let footer4_x = Mm(200.0 + margin);
    //     current_layer.use_text("Kasir,", 8.0, footer4_x, footer_y, &font_regular);

    //     footer_y = footer_y - Mm(18.0);
    //     current_layer.use_text(
    //         "(                                     )",
    //         8.0,
    //         footer1_x - Mm(8.0),
    //         footer_y,
    //         &font_regular,
    //     );

    //     current_layer.use_text(
    //         "(                                     )",
    //         8.0,
    //         footer2_x - Mm(5.0),
    //         footer_y,
    //         &font_regular,
    //     );

    //     current_layer.use_text(
    //         "(                                     )",
    //         8.0,
    //         footer3_x - Mm(2.0),
    //         footer_y,
    //         &font_regular,
    //     );

    //     current_layer.use_text(
    //         "(                                     )",
    //         8.0,
    //         footer4_x - Mm(11.0),
    //         footer_y,
    //         &font_regular,
    //     );
    // }
}
