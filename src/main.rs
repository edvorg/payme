extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate markdown;
extern crate bodyparser;
extern crate persistent;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate tera;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate mime;
#[macro_use] extern crate router;

pub mod payme {
    pub mod redis;
    pub mod json;
    pub mod handler;
    pub mod router;
    pub mod email;
    pub mod crypto;
    pub mod config;
}

use iron::{Iron, Chain};

fn main() {
    println!("Serving on :3000");
    let mut chain = Chain::new(payme::router::make_mount());
    chain.link_before(payme::json::middleware());
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
