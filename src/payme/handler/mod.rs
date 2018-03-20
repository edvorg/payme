extern crate iron;
extern crate params;
extern crate reqwest;

use iron::prelude::*;
use iron::status;
use std::fs::File;
use std::path::Path;
use self::reqwest::Client;
use router::Router;
use payme::json;
use payme::redis;
use payme::email;
use payme::crypto;
use self::params::{Params, Value};

static SECRET: &'static str = "secret";

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
        Some(&Value::String(ref token)) => {
            Some(token)
        },
        _ => {
            None
        },
    }.unwrap().clone()
}

pub fn handle_invoice_request(request: &mut Request) -> IronResult<Response> {
    let map = request.get_ref::<Params>().unwrap();
    let g_recaptcha_response = get_string_param(map, "g-recaptcha-response");
    let info = json::InvoiceInfo {
        task: get_string_param(map, "task"),
        hours: get_string_param(map, "hours"),
        rate: get_string_param(map, "rate"),
        email: get_string_param(map, "email"),
        company: get_string_param(map, "company"),
        company_address: get_string_param(map, "company_address"),
        client_email: get_string_param(map, "client_email"),
        client_company: get_string_param(map, "client_company"),
        client_company_address: get_string_param(map, "client_company_address"),
        terms: get_string_param(map, "terms"),
    };
    let params = [("secret", SECRET),
                  ("response", &g_recaptcha_response)];
    let client = Client::new();
    let res = client.post("https://www.google.com/recaptcha/api/siteverify")
        .form(&params)
        .send()
        .unwrap()
        .json::<json::RecaptchaResponse>()
        .unwrap();
    if res.success {
        println!("sending invoice, {:?}", res);
        let id = redis::get_new_invoice_id();
        redis::set_info(id, info.clone());
        email::send_invoice(id, info.clone());
        let token = crypto::gen_receipt_token(id, info.clone());
        email::send_confirm(id, info.clone(), token);
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
    let invoice_id = request.extensions.get::<Router>().unwrap()
        .find("invoice_id")
        .and_then(|invoice_id| {
            invoice_id.parse::<isize>().ok()
        });
    match invoice_id {
        Some(invoice_id) => {
            let map = request.get_ref::<Params>().unwrap();
            match map.find(&["token"]) {
                Some(&Value::String(ref token)) => {
                    if !redis::is_confirmed(invoice_id) {
                        println!("sending receipt");
                        let info = redis::get_info(invoice_id);
                        match info {
                            Some(info) => {
                                if crypto::is_receipt_token_valid(invoice_id, info.clone(), token.to_string()) {
                                    redis::set_confirmed(invoice_id);
                                    email::send_receipt(invoice_id, info);
                                    redis::del_info(invoice_id);
                                    let mut response = Response::new();
                                    response.set_mut(status::Ok);
                                    response.set_mut("Confirmed");
                                    Ok(response)
                                } else {
                                    let mut response = Response::new();
                                    response.set_mut(status::Ok);
                                    response.set_mut("Invalid token");
                                    Ok(response)
                                }
                            },
                            None => {
                                let mut response = Response::new();
                                response.set_mut(status::Ok);
                                response.set_mut("Invoice not found");
                                Ok(response)
                            },
                        }
                    } else {
                        let mut response = Response::new();
                        response.set_mut(status::Ok);
                        response.set_mut("Receipt already confirmed");
                        Ok(response)
                    }
                },
                _ => {
                    let mut response = Response::new();
                    response.set_mut(status::Ok);
                    response.set_mut("Unable to parse token");
                    Ok(response)
                },
            }
        },
        _ => {
            let mut response = Response::new();
            response.set_mut(status::Ok);
            response.set_mut("Unable to parse invoice_id");
            Ok(response)
        }
    }
}
