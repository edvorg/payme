extern crate iron;
extern crate params;
extern crate reqwest;

use self::params::{Params, Value};
use self::reqwest::Client;
use iron::prelude::*;
use iron::status;
use payme::config;
use payme::crypto;
use payme::email;
use payme::json;
use payme::pdf;
use payme::pdf::PdfType;
use payme::redis;
use router::Router;
use std::fs::File;
use std::i32;
use std::path::Path;
use std::str::FromStr;
use std::thread;

pub fn handle_index_request(_request: &mut Request) -> IronResult<Response> {
    println!("getting index page");
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(File::open(Path::new("web-app/resources/public/index.html")).unwrap());
    Ok(response)
}

fn get_string_param(map: &params::Map, param: &str) -> String {
    match map.find(&[param]) {
        Some(&Value::String(ref token)) => Some(token),
        _ => None,
    }
    .unwrap_or(&"".to_string())
    .clone()
}

fn get_string_param_option(map: &params::Map, param: &str) -> Option<String> {
    match map.find(&[param]) {
        Some(&Value::String(ref token)) => Some(token.to_string()),
        _ => None,
    }
}

fn get_i32_param(map: &params::Map, param: &str) -> i32 {
    match map.find(&[param]) {
        Some(&Value::String(ref token)) => match i32::from_str(token) {
            Ok(v) => Some(v),
            _ => None,
        },
        _ => None,
    }
    .unwrap_or(0)
}

pub fn handle_invoice_request(request: &mut Request) -> IronResult<Response> {
    let map = request.get_ref::<Params>().unwrap();
    let g_recaptcha_response = get_string_param(map, "g-recaptcha-response");
    let info = json::InvoiceInfo {
        task: get_string_param(map, "task"),
        hours: get_i32_param(map, "hours"),
        rate: get_i32_param(map, "rate"),
        email: get_string_param(map, "email"),
        company: get_string_param(map, "company"),
        company_address: get_string_param(map, "company_address"),
        client_email: get_string_param(map, "client_email"),
        client_company: get_string_param(map, "client_company"),
        client_company_address: get_string_param(map, "client_company_address"),
        terms: get_string_param(map, "terms"),
        number: get_i32_param(map, "number"),
        date: get_string_param_option(map, "date"),
    };
    let params = [
        ("secret", &config::get_recaptcha_secret()),
        ("response", &g_recaptcha_response),
    ];
    let client = Client::new();
    let res = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&params)
        .send()
        .unwrap()
        .json::<json::RecaptchaResponse>()
        .unwrap();
    if res.success {
        println!("sending invoice, {:?}", res);
        let id = redis::get_new_invoice_id();
        redis::set_info(id, info.clone());
        thread::spawn(move || {
            let pdf_html = email::render_invoice(PdfType::Invoice, &info);
            pdf::render_pdf_file(PdfType::Invoice, id, info.number, &pdf_html);
            if !redis::is_unsubscribed(info.client_email.clone()) {
                email::send_invoice(id, info.clone());
            }
            let token = crypto::gen_receipt_token(id, info.clone());
            if !redis::is_unsubscribed(info.email.clone()) {
                email::send_invoice_copy(id, info.clone(), token.clone());
            }
            email::send_invoice_diag(id, info.clone(), token);
            pdf::delete_pdf_file(id);
        });
        let mut response = Response::new();
        response.set_mut(status::Ok);
        response.set_mut("Invoiced");
        Ok(response)
    } else {
        println!("unable to validate user");
        let mut response = Response::new();
        response.set_mut(status::Forbidden);
        response.set_mut("Unable to verify user");
        Ok(response)
    }
}

pub fn handle_receipt_request(request: &mut Request) -> IronResult<Response> {
    let invoice_id = request
        .extensions
        .get::<Router>()
        .unwrap()
        .find("invoice_id")
        .and_then(|invoice_id| invoice_id.parse::<isize>().ok());
    match invoice_id {
        Some(invoice_id) => {
            let map = request.get_ref::<Params>().unwrap();
            match map.find(&["token"]) {
                Some(&Value::String(ref token)) => {
                    let info = redis::get_info(invoice_id);
                    match info {
                        Some(info) => {
                            if crypto::is_receipt_token_valid(
                                invoice_id,
                                info.clone(),
                                token.to_string(),
                            ) {
                                if !redis::is_confirmed(invoice_id) {
                                    println!("sending receipt");
                                    redis::set_confirmed(invoice_id);
                                    let token_copy: String = token.clone();
                                    thread::spawn(move || {
                                        let pdf_html =
                                            email::render_invoice(PdfType::Receipt, &info);
                                        pdf::render_pdf_file(
                                            PdfType::Receipt,
                                            invoice_id,
                                            info.number,
                                            &pdf_html,
                                        );
                                        if !redis::is_unsubscribed(info.client_email.clone()) {
                                            email::send_receipt(invoice_id, info.clone());
                                        }
                                        if !redis::is_unsubscribed(info.email.clone()) {
                                            email::send_receipt_copy(invoice_id, info.clone());
                                        }
                                        email::send_receipt_diag(
                                            invoice_id,
                                            info.clone(),
                                            token_copy,
                                        );
                                        pdf::delete_pdf_file(invoice_id);
                                        redis::del_info(invoice_id);
                                    });
                                    let mut response = Response::new();
                                    response.set_mut(status::Ok);
                                    response.set_mut("Confirmed");
                                    Ok(response)
                                } else {
                                    let mut response = Response::new();
                                    response.set_mut(status::Ok);
                                    response.set_mut("Receipt already confirmed");
                                    Ok(response)
                                }
                            } else {
                                let mut response = Response::new();
                                response.set_mut(status::Ok);
                                response.set_mut("Invalid token");
                                Ok(response)
                            }
                        }
                        None => {
                            let mut response = Response::new();
                            response.set_mut(status::Ok);
                            response.set_mut("Invoice not found");
                            Ok(response)
                        }
                    }
                }
                _ => {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("Unable to parse token");
                    Ok(response)
                }
            }
        }
        _ => {
            let mut response = Response::new();
            response.set_mut(status::Ok);
            response.set_mut("Unable to parse invoice_id");
            Ok(response)
        }
    }
}

pub fn handle_unsubscribe_request(request: &mut Request) -> IronResult<Response> {
    let map = request.get_ref::<Params>().unwrap();
    match map.find(&["email"]) {
        Some(&Value::String(ref email)) => match map.find(&["token"]) {
            Some(&Value::String(ref token)) => {
                if crypto::is_unsubscribe_token_valid(email.clone(), token.to_string()) {
                    println!("unsubsibed {}", &email.clone());
                    redis::set_unsubscribed(email.clone());
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("Unsubscribed");
                    Ok(response)
                } else {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("Invalid token");
                    Ok(response)
                }
            }
            _ => {
                let mut response = Response::new();
                response.set_mut(status::Ok);
                response.set_mut("Unable to parse token");
                Ok(response)
            }
        },
        _ => {
            let mut response = Response::new();
            response.set_mut(status::Ok);
            response.set_mut("Unable to parse email");
            Ok(response)
        }
    }
}
