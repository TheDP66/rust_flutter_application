use std::fs::{remove_file, File};
use std::io::Write;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use genpdf::{elements, Scale};
use genpdf::{Document, Size};
use sanitize_filename::sanitize;
use sqlx::MySqlPool;
use uuid::Uuid;

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
        // Load a font from the file system
        let font_family = genpdf::fonts::from_files("./fonts", "Poppins", None)
            .expect("Failed to load font family");

        // Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family);

        doc.set_paper_size(Size::new(300, 100));

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

        // ? Add image
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
                                        .with_alignment(genpdf::Alignment::Center) // Center the image on the page.
                                        .with_scale(Scale::new(0.5, 0.5));

                                    doc.push(image);

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

        let pdf_path = format!("storage/pdf/{}.pdf", filename);

        // Render the document and write it to a file
        let mut pdf_bytes: Vec<u8> = Vec::new();
        let _ = doc.render_to_file(&pdf_path);

        // Read the PDF file and send it as a response
        let mut file = std::fs::File::open(&pdf_path).unwrap();
        let mut buffer = Vec::new();
        std::io::copy(&mut file, &mut buffer).unwrap();

        Ok(buffer)
    }
}
