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
use std::os;
use std::path::PathBuf;
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

    let mut cert_path = os::self_exe_path().unwrap();
    let mut key_path = cert_path.clone();
    let cert_conf = api.config.get_value_or::<String>("Security.certificate", "snakeoil.key".to_string());
    cert_path.push(cert_conf);
    let key_conf = api.config.get_value_or::<String>("Security.key", "snakeoil.cert".to_string());
    key_path.push(key_conf);

    println!("{}", key_path.as_str().unwrap());

	Iron::new(chain).https(sock_addr, cert_path, key_path).unwrap();
}

#[cfg(test)]
mod tests
{
    use super::*;

    use std::old_io::util::NullReader;
    use std::old_io::net::ip::{SocketAddr, IpAddr};
    use hyper::http::HttpReader::EmptyReader;
    use iron::request::{Request, Url};
    use iron::headers::Headers;
    use iron::request::Body;
    use iron::method::Method;
    use iron::TypeMap;
    pub fn create_fake_request(method: Method, url: &str) // -> Request
    {
        let addr: IpAddr = "localhost".parse().unwrap();
        /* Request
        {
            url: Url::parse(url).unwrap(),
            remote_addr: SocketAddr { ip: addr, port: 8123 },
            local_addr: SocketAddr { ip: addr, port: 3000 },
            headers: Headers::new(),
            body: Body::new(EmptyReader(MockStream::new())),
            method: Method::Get,
            extensions: TypeMap::new()
        } */
    }
}

