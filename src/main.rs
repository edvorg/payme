extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate markdown;
extern crate bodyparser;
extern crate persistent;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate tera;
#[macro_use] extern crate mime;
#[macro_use] extern crate router;
#[macro_use] extern crate lazy_static;

use std::fs::File;
use std::path::Path;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use staticfile::Static;
use tera::Tera;
use tera::Context;

lazy_static! {
    pub static ref TERA: Tera = {
        #[allow(unused_mut)]
        let mut tera = compile_templates!("web-app/resources/public/templates/*");
        tera.autoescape_on(vec![]);
        #[warn(unused_mut)]
        tera
    };
}

fn render_post(post_id: u32) -> String {
    File::open(format!("web-app/resources/public/markdown/{}.md", post_id)).and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s).map(|_size| s)
        }).map(|post: String| {
            markdown::to_html(&post)
        }).map(|post| {
            let mut context = Context::new();
            context.add("post", &post);
            TERA.render("post.html", &context)
                .or_else(|e| {
                    println!("error {}", e);
                    Err(e)
                }).unwrap_or(String::from("can not render"))
        }).unwrap_or(String::from("Post not found"))
}

fn handle_post_request(request: &mut Request) -> IronResult<Response> {
    let post_id = request.extensions.get::<Router>().unwrap()
        .find("post_id")
        .and_then(|post_id| {
            post_id.parse::<u32>().ok()
        });
    let response_text = post_id.map(|post_id| {
        render_post(post_id)
    }).unwrap_or(String::from("Invalid post id"));
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(response_text);
    Ok(response)
}

fn handle_index_request(_request: &mut Request) -> IronResult<Response> {
    println!("getting index page");
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(File::open(Path::new("web-app/resources/public/index.html")).unwrap());
    Ok(response)
}

#[derive(Debug, Clone, Deserialize)]
struct InvoiceInfo {
    task: String,
    hours: String,
    rate: String,
    email: String,
    company: String,
    company_address: String,
    client_email: String,
    client_company: String,
    client_company_address: String,
    terms: String,
}

fn handle_invoice_request(request: &mut Request) -> IronResult<Response> {
    let struct_body = request.get::<bodyparser::Struct<InvoiceInfo>>();
    match struct_body {
        Ok(Some(struct_body)) => println!("Parsed body:\n{:?}", struct_body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    println!("sending invoice");
    let mut response = Response::new();
    response.set_mut(status::Ok);
    Ok(response)
}

fn make_index_router() -> Router {
    router!(
        index: get "/" => handle_index_request
    )
}

fn make_invoice_router() -> Router {
    router!(
        invoice: post "/" => handle_invoice_request
    )
}

fn make_mount() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/", make_index_router());
    mount.mount("/invoice/", make_invoice_router());
    mount.mount("/js/", Static::new(Path::new("web-app/resources/public/js/")));
    mount.mount("/css/", Static::new(Path::new("web-app/resources/public/css/")));
    mount.mount("/scss/", Static::new(Path::new("web-app/resources/public/scss/")));
    mount
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    println!("Serving on :3000");
    let mut chain = Chain::new(make_mount());
    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}
