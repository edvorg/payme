extern crate iron;

use iron::prelude::*;
use iron::status;
use std::fs::File;
use std::path::Path;

use router::Router;
use payme::invoice;
use payme::redis;
use payme::email;

pub fn handle_index_request(_request: &mut Request) -> IronResult<Response> {
    println!("getting index page");
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(File::open(Path::new("web-app/resources/public/index.html")).unwrap());
    Ok(response)
}

pub fn handle_invoice_request(request: &mut Request) -> IronResult<Response> {
    let struct_body = invoice::parse(request);
    match struct_body {
        Ok(Some(struct_body)) => {
            println!("sending invoice");
            let id = redis::get_new_invoice_id();
            redis::set_info(id, struct_body.clone());
            email::send_invoice(id, struct_body.clone());
            email::send_confirm(id, struct_body.clone());
        },
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut("Invoiced");
    Ok(response)
}

pub fn handle_receipt_request(request: &mut Request) -> IronResult<Response> {
    let invoice_id = request.extensions.get::<Router>().unwrap()
        .find("invoice_id")
        .and_then(|invoice_id| {
            invoice_id.parse::<isize>().ok()
        }).unwrap();
    if !redis::is_confirmed(invoice_id) {
        println!("sending receipt");
        let info = redis::get_info(invoice_id);
        match info {
            Some(info) => {
                redis::set_confirmed(invoice_id);
                email::send_receipt(invoice_id, info);
                redis::del_info(invoice_id);
            },
            None => println!("Invoice not found "),
        }
    } else {
        println!("have already sent receipt");
    }
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut("Confirmed");
    Ok(response)
}
