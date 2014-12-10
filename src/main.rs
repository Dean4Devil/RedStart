#![feature(globs)]

extern crate iron;
extern crate url;
extern crate queryst;
extern crate serialize;
extern crate toml;

use iron::prelude::*;

use iron::ChainBuilder;

use controller::Reservation;

use redstart::ConfigReader;
use redstart::URLParser;
use redstart::PermCheck;
use redstart::Logger;
use redstart::RedStart;

mod controller;

mod model;

mod redstart;

fn setup()
{
  let mut config_reader = ConfigReader::new();
  let value = config_reader.get_string("General.name").unwrap();
  println!("{}: Config Loaded", value);
}

fn main()
{
    setup();
    let mut chain = ChainBuilder::new(RedStart);
    chain.link_before(URLParser);
    chain.link_before(PermCheck);
    let mut logger = Logger::new("log.txt");
    chain.link_after(logger);

    Iron::new(chain).listen("localhost:3000").unwrap();
    println!("On 3000");
}
