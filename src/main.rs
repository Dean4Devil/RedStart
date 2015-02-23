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

//use redstart::ConfigReader;
use redstart::Store;
||||||| merged common ancestors
//use redstart::ConfigReader;
use redstart::URLParser;
use redstart::CookieParser;
//use redstart::CookieSetter;
//use redstart::PermCheck;
//use redstart::Logger;
use redstart::RedStart;

mod controller;
mod model;
mod configreader;
mod redstart;

fn setup() -> iron::Chain
{
    let cookieparser = CookieParser::new(Store::new());
||||||| merged common ancestors
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
