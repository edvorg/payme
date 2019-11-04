extern crate chrono;
extern crate markdown;
extern crate serde_json;

use self::chrono::Local;
use payme::config;
use payme::crypto;
use payme::json;
use payme::pdf;
use payme::pdf::get_pdf_title;
use payme::pdf::PdfType;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use tera::Context;
use tera::Tera;

lazy_static! {
    pub static ref TERA: Tera = {
        #[allow(unused_mut)]
        let mut tera = compile_templates!("web-app/resources/public/templates/*");
        tera.autoescape_on(vec![]);
        #[warn(unused_mut)]
        tera
    };
}

fn render_markdown(email_id: String) -> Result<String, Error> {
    File::open(format!("web-app/resources/public/markdown/{}.md", email_id))
        .and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s).map(|_size| s)
        })
        .map(|email: String| markdown::to_html(&email))
}

#[test]
fn render_markdown_test() {
    assert_eq!("<h1 id=\'heading\'>Heading</h1>\n\n<p>Hello {{receiver}}</p>\n\n<p><a href=\'google.com\'>unsubscribe</a></p>\n".to_string(),
               render_markdown("test".to_string()).unwrap_or("not_found".to_string()));
}

pub fn render_invoice(pdf_type: PdfType, info: &json::InvoiceInfo) -> String {
    let mut style = "".to_string();
    let date = Local::now();
    let datestr = match &pdf_type {
        &PdfType::Invoice => info
            .date
            .clone()
            .unwrap_or_else(|| format!("{}", date.format("%b %d, %Y"))),
        &PdfType::Receipt => format!("{}", date.format("%b %d, %Y")),
    };
    let title = get_pdf_title(pdf_type);
    File::open(Path::new("web-app/resources/public/css/invoice.css"))
        .unwrap()
        .read_to_string(&mut style)
        .unwrap();
    let mut context = Context::new();
    context.add("date", &datestr);
    context.add("style", &style);
    context.add("title", &title);
    context.add("number", &info.number);
    context.add("company", &info.company);
    context.add("company_address", &info.company_address);
    context.add("client_company", &info.client_company);
    context.add("client_company_address", &info.client_company_address);
    context.add("task", &info.task);
    context.add("hours", &info.hours);
    context.add("rate", &format!("${}.00", info.rate));
    context.add("amount", &format!("${}.00", info.hours * info.rate));
    context.add(
        "terms",
        &format!(
            "<p>{}</p>",
            info.terms.clone().replace("<ENTER><ENTER>", "</p><p>")
        ),
    );
    TERA.render("invoice.html", &context)
        .or_else(|e| {
            println!("error {}", e);
            Err(e)
        })
        .unwrap_or("Can not render".to_string())
}

fn render_email(
    template_id: String,
    email_id: String,
    user: String,
    receiver: String,
    unsubscribe_email: String,
    token: String,
    invoice_id: String,
) -> String {
    let unsubscribe_token = crypto::gen_unsubscribe_token(unsubscribe_email.clone());
    render_markdown(email_id)
        .map(|email_html| {
            email_html
                .replace("{{user}}", &user)
                .replace("{{receiver}}", &receiver)
                .replace("{{unsubscribe_email}}", &unsubscribe_email)
                .replace("{{unsubscribe_token}}", &unsubscribe_token)
                .replace("{{token}}", &token)
                .replace("{{invoice_id}}", &invoice_id)
                .replace("{{host}}", &config::get_host())
        })
        .map(|email_html| {
            let mut context = Context::new();
            context.add("email", &email_html);
            context.add("user", &user);
            context.add("receiver", &receiver);
            context.add("unsubscribe_email", &unsubscribe_email);
            context.add("unsubscribe_token", &unsubscribe_token);
            context.add("token", &token);
            context.add("invoice_id", &invoice_id);
            context.add("host", &config::get_host());
            TERA.render(&format!("{}.html", template_id), &context)
                .or_else(|e| {
                    println!("error {}", e);
                    Err(e)
                })
                .unwrap_or(String::from("Can not render"))
        })
        .unwrap_or(String::from("Email not found"))
}

#[test]
fn render_email_test() {
    assert_eq!("<a>open test user</a>\n<h1 id=\'heading\'>Heading</h1>\n\n<p>Hello test receiver</p>\n\n<p><a href=\'google.com\'>unsubscribe</a></p>\n\n<a>close</a>\n".to_string(),
               render_email("test".to_string(),
                            "test".to_string(),
                            "test user".to_string(),
                            "test receiver".to_string(),
                            "".to_string(),
                            "".to_string(),
                            "".to_string()));
}

pub fn send_invoice(invoice_id: isize, invoice: json::InvoiceInfo) {
    let output = render_email(
        "email".to_string(),
        "invoice".to_string(),
        invoice.company.clone(),
        invoice.client_company.clone(),
        invoice.client_email.clone(),
        "".to_string(),
        format!("{}", invoice_id),
    );
    println!("sending email to {} {}", &invoice.client_email, &output);
    let mut command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-s")
        .arg(format!("Invoice #{}", invoice.number))
        .arg("-a")
        .arg(pdf::get_pdf_path(
            PdfType::Invoice,
            invoice_id,
            invoice.number,
        ))
        .arg("--")
        .arg(invoice.client_email)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(command.stdin.as_mut().unwrap(), "{}", output).unwrap();
    command.wait().unwrap();
}

pub fn send_invoice_copy(invoice_id: isize, invoice: json::InvoiceInfo, token: String) {
    let output = render_email(
        "email".to_string(),
        "invoice_copy".to_string(),
        invoice.client_company.clone(),
        invoice.company.clone(),
        invoice.email.clone(),
        token,
        format!("{}", invoice_id),
    );
    println!("sending email to {} {}", &invoice.email, &output);
    let mut command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-s")
        .arg(format!("Invoice #{}", invoice.number))
        .arg("-a")
        .arg(pdf::get_pdf_path(
            PdfType::Invoice,
            invoice_id,
            invoice.number,
        ))
        .arg("--")
        .arg(invoice.email)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(command.stdin.as_mut().unwrap(), "{}", output).unwrap();
    command.wait().unwrap();
}

pub fn send_invoice_diag(invoice_id: isize, invoice: json::InvoiceInfo, token: String) {
    let output = render_email(
        "email".to_string(),
        "invoice_diag".to_string(),
        invoice.company,
        invoice.client_company,
        invoice.email.clone(),
        token,
        format!("{}", invoice_id),
    );
    println!(
        "sending email to {} {}",
        &"payme@rust.cafe".to_string(),
        &output
    );
    let mut command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-s")
        .arg("Invoice")
        .arg("--")
        .arg("payme@rust.cafe".to_string())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(command.stdin.as_mut().unwrap(), "{}", output).unwrap();
    command.wait().unwrap();
}

pub fn send_receipt_diag(invoice_id: isize, invoice: json::InvoiceInfo, token: String) {
    let output = render_email(
        "email".to_string(),
        "receipt_diag".to_string(),
        invoice.company,
        invoice.client_company,
        invoice.email.clone(),
        token,
        format!("{}", invoice_id),
    );
    println!(
        "sending email to {} {}",
        &"payme@rust.cafe".to_string(),
        &output
    );
    let mut command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-s")
        .arg("Receipt")
        .arg("--")
        .arg("payme@rust.cafe".to_string())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(command.stdin.as_mut().unwrap(), "{}", output).unwrap();
    command.wait().unwrap();
}

pub fn send_receipt(invoice_id: isize, invoice: json::InvoiceInfo) {
    let output = render_email(
        "email".to_string(),
        "receipt".to_string(),
        invoice.company.clone(),
        invoice.client_company.clone(),
        invoice.client_email.clone(),
        "".to_string(),
        format!("{}", invoice_id),
    );
    println!("sending email to {} {}", &invoice.client_email, &output);
    let mut command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-s")
        .arg(format!("Receipt #{}", invoice.number))
        .arg("-a")
        .arg(pdf::get_pdf_path(
            PdfType::Receipt,
            invoice_id,
            invoice.number,
        ))
        .arg("--")
        .arg(invoice.client_email)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(command.stdin.as_mut().unwrap(), "{}", output).unwrap();
    command.wait().unwrap();
}

pub fn send_receipt_copy(invoice_id: isize, invoice: json::InvoiceInfo) {
    let output = render_email(
        "email".to_string(),
        "receipt_copy".to_string(),
        invoice.client_company.clone(),
        invoice.company.clone(),
        invoice.email.clone(),
        "".to_string(),
        format!("{}", invoice_id),
    );
    println!("sending email to {} {}", &invoice.email, &output);
    let mut command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-s")
        .arg(format!("Receipt #{}", invoice.number))
        .arg("-a")
        .arg(pdf::get_pdf_path(
            PdfType::Receipt,
            invoice_id,
            invoice.number,
        ))
        .arg("--")
        .arg(invoice.email)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(command.stdin.as_mut().unwrap(), "{}", output).unwrap();
    command.wait().unwrap();
}
