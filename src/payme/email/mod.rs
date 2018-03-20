extern crate markdown;
extern crate serde_json;

use std::fs::File;
use std::io::{Write, Read, Error};
use tera::Tera;
use tera::Context;
use std::process::{Command, Stdio};
use payme::config;
use payme::json;

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
    File::open(format!("web-app/resources/public/markdown/{}.md", email_id)).and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).map(|_size| s)
    }).map(|email: String| {
        markdown::to_html(&email)
    })
}

#[test]
fn render_markdown_test() {
    assert_eq!("<h1 id=\'heading\'>Heading</h1>\n\n<p>Hello {{receiver}}</p>\n\n<p><a href=\'google.com\'>unsubscribe</a></p>\n".to_string(),
               render_markdown("test".to_string()).unwrap_or("not_found".to_string()));

}

fn render_email(template_id: String,
                email_id: String,
                user: String,
                receiver: String,
                invoice_id: String,
                token: String,
                content: String) -> String {
    render_markdown(email_id)
        .map(|email| {
            email.replace("{{user}}", &user)
                .replace("{{receiver}}", &receiver)
                .replace("{{invoice_id}}", &invoice_id)
                .replace("{{token}}", &token)
                .replace("{{content}}", &content)
                .replace("{{host}}", &config::get_host())
        }).map(|email| {
            let mut context = Context::new();
            context.add("email", &email);
            context.add("user", &user);
            context.add("invoice_id", &invoice_id);
            context.add("token", &token);
            context.add("content", &content);
            context.add("host", &config::get_host());
            TERA.render(&format!("{}.html", template_id), &context)
                .or_else(|e| {
                    println!("error {}", e);
                    Err(e)
                }).unwrap_or(String::from("Can not render"))
        }).unwrap_or(String::from("Email not found"))
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
    let output = render_email("email".to_string(),
                              "invoice".to_string(),
                              invoice.company.clone(),
                              invoice.client_company.clone(),
                              format!("{}", invoice_id),
                              "".to_string(),
                              serde_json::to_string(&invoice).unwrap());
    println!("sending email to {} {}", &invoice.client_email, &output);
    let put_command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-c")
        .arg(&invoice.email)
        .arg("-s")
        .arg("Invoice")
        .arg("--")
        .arg(invoice.client_email)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(put_command.stdin.unwrap(), "{}", output).unwrap();
}

pub fn send_confirm(invoice_id: isize, invoice: json::InvoiceInfo, token: String) {
    let output = render_email("email".to_string(),
                              "confirm".to_string(),
                              invoice.company,
                              invoice.client_company,
                              format!("{}", invoice_id),
                              token,
                              "".to_string());
    println!("sending email to {} {}", &invoice.email, &output);
    let put_command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-c")
        .arg("payme@rust.cafe")
        .arg("-s")
        .arg("Invoice confirmation")
        .arg("--")
        .arg(invoice.email)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(put_command.stdin.unwrap(), "{}", output).unwrap();
}

pub fn send_receipt(invoice_id: isize, invoice: json::InvoiceInfo) {
    let output = render_email("email".to_string(),
                              "receipt".to_string(),
                              invoice.company.clone(),
                              invoice.client_company.clone(),
                              format!("{}", invoice_id),
                              "".to_string(),
                              serde_json::to_string(&invoice).unwrap());
    println!("sending email to {} {}", &invoice.client_email, &output);
    let put_command = Command::new("mutt")
        .arg("-e")
        .arg("set content_type=text/html")
        .arg("-c")
        .arg(&invoice.email)
        .arg("-s")
        .arg("Receipt")
        .arg("--")
        .arg(invoice.client_email)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    write!(put_command.stdin.unwrap(), "{}", output).unwrap();
}
