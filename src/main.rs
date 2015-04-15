#![crate_type = "bin"]
#![crate_name = "RedStart"]

/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein, Harald Seiler, Maximillian Zander
 */

//! The Entry point of RedStart.
//! This Module contains the 'setup' function and the 'main' function that are run in this order

#![allow(non_snake_case)]
// TODO: Remove the need for this compilation attribute.
#![allow(dead_code)]
extern crate iron;
extern crate hyper;
extern crate url;
extern crate toml;
extern crate cookie;
extern crate ldap;
extern crate rustc_serialize as serialize;
extern crate rand;

//use std::error::Error;
use std::os;
use std::env;
use std::thread;
use std::path::PathBuf;
use std::net::{SocketAddrV4, Ipv4Addr};

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

/// The setup function
///
/// This function sets up the environment needed for RedStart's main loop to function.
/// The main work this function does is building the chain and populating the API.
fn setup() -> (API, iron::Chain)
{
    let mut api = API::new();
    let redstart = RedStart::new(&api);
    let cookieparser = CookieParser::new(&api);
    let cookesetter = CookieSetter::new(&api);
    let mut chain = Chain::new(redstart);
    chain.link_before(URLParser);
    chain.link_before(cookieparser);
    chain.link_after(cookesetter);
    return (api, chain);
}

/// The main function
///
/// This is the main loop of RedStart
fn main()
{
    let (mut api, chain) = setup();

    let addr: Ipv4Addr = api.config.get_value_or::<String>("Networking.address", "localhost".to_string()).parse().unwrap();
    let port = api.config.get_value_or::<u16>("Networking.port", 3000);
    let sock_addr = SocketAddrV4::new(addr, port);

    let guards; // JoinGuards that will make sure the child processes don't get dropped when the main thread is finished.

    // Is HTTPS enabled?
    if api.config.get_value_or::<bool>("Security.https", false)
    {
        // If yes, get two paths located at the current pwd.
        let mut cert_path = env::current_exe().unwrap();
        let mut key_path = cert_path.clone();

        // Get the location of the Certificate file.
        let cert_conf = api.config.get_value_or::<String>("Security.certificate", "snakeoil.key".to_string());
        cert_path.push(cert_conf);

        // Get the location of the Key file.
        let key_conf = api.config.get_value_or::<String>("Security.key", "snakeoil.cert".to_string());
        key_path.push(key_conf);

        // Start up Iron in HTTPS mode
        guards = Iron::new(chain).https(sock_addr, cert_path, key_path).unwrap();
    }
    else
    {
        // If no, start up Iron in HTTP mode.
        guards = Iron::new(chain).http(sock_addr).unwrap();
    }

    println!("Started RedStart on port {}", port);
}


#[cfg(test)]
/// Test module
///
/// This module only gets compiled in test mode.
/// It provides mostly helper functions for other unit tests.
mod tests
{
    use super::*;

    use std::old_io::util::NullReader;
    use std::old_io::net::ip::{SocketAddr, IpAddr};
    use hyper::http::HttpReader;
    use iron::request::{Request, Url};
    use iron::headers::Headers;
    use iron::request::Body;
    use iron::method::Method;
    use iron::TypeMap;

    pub fn create_fake_request(method: Method, url: &str) -> Request
    {
        let addr: IpAddr = "localhost".parse().unwrap();
        Request
        {
            url: Url::parse(url).unwrap(),
            remote_addr: SocketAddr { ip: addr, port: 8123 },
            local_addr: SocketAddr { ip: addr, port: 3000 },
            headers: Headers::new(),
            body: Body::new(HttpReader::EmptyReader::<NullReader>::new()),
            method: Method::Get,
            extensions: TypeMap::new()
        }
    }
}

