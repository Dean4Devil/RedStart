#![feature(globs)]

extern crate iron;

use iron::prelude::*;

use iron::ChainBuilder;

use redstart::Router;
use redstart::Logger;
use redstart::RedStart;

mod redstart;

fn main() {
    let mut chain = ChainBuilder::new(RedStart);
    chain.link_before(Router);
    let mut logger = Logger::new("log.txt");
    chain.link_after(logger); 

    Iron::new(chain).listen("localhost:3000").unwrap();
    println!("On 3000");
}
