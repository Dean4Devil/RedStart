#![feature(macro_rules, phase)]

extern crate hyper;

use std::io::net::ip::Ipv4Addr;

use hyper::{Get, Post};
use hyper::header::common::ContentLength;
use hyper::server::{Server, Request, Response};

use redstart::serve;

mod redstart;

fn main() {
    let server = Server::http(Ipv4Addr(127,0,0,1), 8080);
    server.listen(serve).unwrap();
}
