use iron::prelude::*;
use iron::{BeforeMiddleware, status};
use iron::typemap::Key;

use std::error::Error;
use std::fmt::{self, Debug};

use queryst;
use serialize::json;

// Errors for the win!
#[derive(Debug)]
pub struct MalformedRequest(String);
#[derive(Debug)]
pub struct NotFound(String);

impl Error for MalformedRequest
{
    fn description(&self) -> &'static str { "MalformedRequest" }
}

impl fmt::Display for MalformedRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotFound
{
    fn description(&self) -> &'static str { "NoRoute" }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub struct URL;

impl Key for URL { type Value = [&'static str; 2]; }

pub struct URLParser;

// Make URLParser a BeforeMiddleware
impl BeforeMiddleware for URLParser
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        /*
        URL Structure:
            * No path defined
            * query string contains an r variable
            * r value consits of two subvalues seperated by a slash
         */

        // TODO: Figure out Lifetimes.
        let req_url: [&'static str; 2] = ["reservation", "timetable"];
        req.extensions.insert::<URL>(req_url);

        Ok(())
    }
}
