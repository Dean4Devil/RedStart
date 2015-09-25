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
//#![allow(dead_code)]
extern crate iron;
extern crate hyper;
extern crate url;
extern crate toml;
extern crate cookie;
extern crate ldap;
extern crate rustc_serialize as serialize;
extern crate rand;
extern crate mysql;

//use std::error::Error;
use std::env;
use std::net::{SocketAddrV4, Ipv4Addr};

use iron::prelude::*;
//use iron::AroundMiddleware;

macro_rules! dbgprint {
    ($fmt:expr) => (if cfg!(debug){println!($fmt)});
    ($fmt:expr, $($arg:tt)*) => (if cfg!(debug){println!($fmt, $($arg)*)});
}

use api::API;
use urlparser::URLParser;
use cookieparser::CookieParser;
use cookiesetter::CookieSetter;
use redstart::RedStart;

use controller::{User, Group, Reservation};

mod api;
mod authentication;
mod controller;
mod data;
mod model;
mod configreader;
mod cookiesetter;
mod cookieparser;
mod urlparser;
mod session;
mod permissions;
mod redstart;


/// The setup function
///
/// This function sets up the environment needed for RedStart's main loop to function.
/// The main work this function does is building the chain and populating the API.
fn setup() -> (API, iron::Chain)
{
    dbgprint!("Starting Setup phase");
    let api = API::new();

    // Create Controller Tree
    let mut redstart = RedStart::new(&api);
    redstart.add_controller(Box::new(controller::User::new(&api)));
    redstart.add_controller(Box::new(controller::Group::new(&api)));
    //redstart.add_controller(Box::new(controller::Reservation::new(&api)));
    redstart.finish();
    // Make RedStart unmutable
    let redstart = redstart;

    let cookieparser = CookieParser::new(&api);
    let cookesetter = CookieSetter::new(&api);

    let mut chain = Chain::new(redstart);
    chain.link_before(URLParser);
    chain.link_before(cookieparser);
    chain.link_after(cookesetter);
    dbgprint!("Setup has finished");
    return (api, chain);
}

/// The main function
///
/// This is the main loop of RedStart
fn main()
{
    // The join guards are "unused", but have a reason!
    #![allow(unused_variables, unused_assignments)]


    let (mut api, chain) = setup();

    let addr: Ipv4Addr = api.config.get_value_or::<String>("Networking.address", "localhost".to_string()).parse().unwrap();
    let port = api.config.get_value_or::<u16>("Networking.port", 3000);
    let sock_addr = SocketAddrV4::new(addr, port);

    let guards; // JoinGuards that will make sure the child processes don't get dropped when the main thread is finished.

    // Is HTTPS enabled?
    if api.config.get_value_or::<bool>("Security.https", false)
    {
        dbgprint!("HTTPS is enabled! Setting up...");
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

    dbgprint!("Started RedStart on port {}", port);
}


#[cfg(test)]
/// Test module
///
/// This module only gets compiled in test mode.
/// It provides mostly helper functions for other unit tests.
mod tests
{
    use super::*;

    use hyper::http::HttpReader;
    use iron::request::{Request, Url};
    use iron::headers::Headers;
    use iron::request::Body;
    use iron::method::Method;
    use iron::TypeMap;
}

