#![feature(globs)]

extern crate iron;
extern crate "route-recognizer" as recognizer;

use iron::prelude::*;

use iron::ChainBuilder;

use redstart::Router;
use redstart::PermCheck;
use redstart::Logger;
use redstart::RedStart;

mod redstart;

fn main() {
    let mut chain = ChainBuilder::new(RedStart);
    chain.link_before(Router);
    chain.link_before(PermCheck);
    let mut logger = Logger::new("log.txt");
    chain.link_after(logger); 

    Iron::new(chain).listen("localhost:3000").unwrap();
    println!("On 3000");
}
