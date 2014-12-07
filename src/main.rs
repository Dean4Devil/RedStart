#![feature(globs)]

extern crate iron;
extern crate url;
extern crate queryst;
extern crate serialize;
extern crate toml;

use iron::prelude::*;

use iron::ChainBuilder;

use controller::Reservation;

use redstart::URLParser;
use redstart::PermCheck;
use redstart::Logger;
use redstart::RedStart;

mod controller;

mod model;

mod redstart;

fn setup()
{
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
