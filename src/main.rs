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
use std::str::FromStr;
use std::old_io::net::ip::{ToSocketAddr, SocketAddr, IpAddr};

use iron::prelude::*;
//use iron::AroundMiddleware;

//use controller::Reservation;

use configreader::ConfigReader;
use redstart::API;
use redstart::URLParser;
use redstart::CookieParser;
use redstart::CookieSetter;
//use redstart::CookieSetter;
//use redstart::PermCheck;
//use redstart::Logger;
use redstart::Store;
use redstart::RedStart;

mod controller;
mod model;
mod configreader;
mod redstart;

fn setup() -> (API, iron::Chain)
{
    let mut api = API::new();
    let serve_name = api.config.get_value_or::<String>("General.name", "PeachesDev RedStart".to_string());
    let address = api.config.get_value_or::<String>("Networking.address", "localhost".to_string());
    let port = api.config.get_value_or::<i32>("Networking.port", 3000);
    println!("{} starting on {}:{}", serve_name, address, port);
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
