extern crate iron;
extern crate hyper;
extern crate url;
extern crate queryst;
extern crate serialize;
extern crate toml;
extern crate cookie;

use std::error::Error;

use iron::prelude::*;
use iron::AroundMiddleware;

use controller::Reservation;

use redstart::ConfigReader;
use redstart::URLParser;
use redstart::CookieParser;
use redstart::PermCheck;
use redstart::Logger;
use redstart::{RedStart, RedStartCatch};

mod controller;

mod model;

mod redstart;

fn setup()
{
  //let mut config_reader = ConfigReader::new();
  //let value = config_reader.get_string("General.name").unwrap();
  //println!("{}: Config Loaded", value);
}

fn main()
{
    setup();
    let mut chain = Chain::new(RedStart);
    chain.link_before(URLParser);
    chain.link_before(CookieParser);
    //chain.link_before(PermCheck);
    chain.link_after(RedStartCatch);
    let mut logger = Logger::new("log.txt");
    chain.link_after(logger);

    Iron::new(chain).http("localhost:3000").unwrap();
    println!("On 3000");
}
