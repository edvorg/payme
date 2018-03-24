extern crate reqwest;

use self::reqwest::Client;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::fs;

#[derive(Clone)]
pub enum PdfType {
    Invoice,
    Receipt,
}

pub fn get_pdf_title(pdf_type: PdfType) -> String {
    match pdf_type {
        PdfType::Invoice => "invoice".to_string(),
        PdfType::Receipt => "receipt".to_string(),
    }
}

fn get_pdf_dir_path(id: isize) -> String {
    format!("./pdf/{}", id)
}

pub fn get_pdf_path(pdf_type: PdfType, id: isize, number: i32) -> String {
    format!("{}/{}-{}.pdf", get_pdf_dir_path(id), get_pdf_title(pdf_type), number)
}

pub fn render_pdf_file(pdf_type: PdfType, id: isize, number: i32, content: &String) {
    let client = Client::new();
    let mut buf: Vec<u8> = vec![];
    client.post("http://127.0.0.1:5001/pdf")
        .body(content.clone())
        .send()
        .unwrap()
        .copy_to(&mut buf)
        .unwrap();
    fs::create_dir_all(Path::new(&get_pdf_dir_path(id))).unwrap();
    let mut f = File::create(Path::new(&get_pdf_path(pdf_type.clone(), id, number))).unwrap();
    f.write_all(&mut buf).unwrap();
    f.flush().unwrap();
}

pub fn delete_pdf_file(pdf_type: PdfType, id: isize, number: i32) {
    fs::remove_file(Path::new(&get_pdf_path(pdf_type, id, number))).unwrap();
    fs::remove_dir(Path::new(&get_pdf_dir_path(id))).unwrap();
}
