extern crate bodyparser;
extern crate iron;
extern crate markdown;
extern crate mount;
extern crate persistent;
extern crate staticfile;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate mime;
#[macro_use]
extern crate router;

pub mod payme {
    pub mod config;
    pub mod crypto;
    pub mod email;
    pub mod handler;
    pub mod json;
    pub mod pdf;
    pub mod redis;
    pub mod router;
}

use iron::{Chain, Iron};

fn main() {
    println!("Serving on :3000");
    let mut chain = Chain::new(payme::router::make_mount());
    chain.link_before(payme::json::middleware());
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
