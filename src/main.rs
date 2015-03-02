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
use std::old_io::net::ip::{SocketAddr, IpAddr};

use iron::prelude::*;
//use iron::AroundMiddleware;

//use controller::Reservation;

use api::API;
use urlparser::URLParser;
use cookieparser::CookieParser;
use cookiesetter::CookieSetter;
use redstart::RedStart;

mod api;
mod authentication;
mod controller;
mod model;
mod configreader;
mod cookiesetter;
mod cookieparser;
mod urlparser;
mod session;
mod redstart;

fn setup() -> (API, iron::Chain)
{
    let mut api = API::new();
    let redstart = RedStart::new(api.sessions.clone());
    let cookieparser = CookieParser::new(api.sessions.clone());
    let cookesetter = CookieSetter::new(api.sessions.clone());
	let mut chain = Chain::new(redstart);
	chain.link_before(URLParser);
	chain.link_before(cookieparser);
    chain.link_after(cookesetter);
	return (api, chain);
}

fn main()
{
	let (mut api, chain) = setup();
    let addr: IpAddr = api.config.get_value_or::<String>("Networking.address", "localhost".to_string()).parse().unwrap();
    let port = api.config.get_value_or::<u16>("Networking.port", 3000);
    let sock_addr = SocketAddr { ip: addr, port: port };
	Iron::new(chain).http(sock_addr).unwrap();
}
