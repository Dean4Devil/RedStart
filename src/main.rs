#![feature(globs)]

extern crate iron;
extern crate router;

use iron::prelude::*;

use router::Router;

use redstart::serve;

mod redstart;

fn main() {
    let mut router = Router::new();
    router.get("/:controller/:model", serve);

    Iron::new(router).listen("localhost:3000").unwrap();
    println!("On 3000");
}
