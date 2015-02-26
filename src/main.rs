#![allow(unstable_features)]
#![allow(non_snake_case)]
// Since both ConfigReader and Logger are not functional yet allow dead code.
#![allow(dead_code)]
#![feature(core,old_io,old_path)]
extern crate iron;
extern crate hyper;
extern crate url;
extern crate "rustc-serialize" as serialize;
extern crate toml;
extern crate cookie;

//use std::error::Error;

use iron::prelude::*;
//use iron::AroundMiddleware;

//use controller::Reservation;

use configreader::ConfigReader;
use redstart::URLParser;
use redstart::CookieParser;
//use redstart::CookieSetter;
//use redstart::PermCheck;
//use redstart::Logger;
use redstart::Store;
use redstart::RedStart;

mod controller;
mod model;
mod configreader;
mod redstart;

fn setup() -> iron::Chain
{
    let config = ConfigReader::new();
    let cookieparser = CookieParser::new(Store::new());
	let mut chain = Chain::new(RedStart);
	chain.link_before(URLParser);
	chain.link_before(cookieparser);
	return chain;
}

fn main()
{
	let chain: iron::Chain = setup();
	Iron::new(chain).http("localhost:3000").unwrap();
}
