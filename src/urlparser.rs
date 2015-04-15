/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein, Maximillian Zander
 */

use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::typemap::Key;
use iron::status;

use std::error::Error;
use std::fmt::{self, Debug};

// Errors for the win!
#[derive(Debug)]
pub struct MalformedRequest;
impl Error for MalformedRequest
{
    fn description(&self) -> &'static str { "MalformedRequest" }
}

impl fmt::Display for MalformedRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

// Struct that is inserted as Key into req.extensions
pub struct URL;
impl Key for URL { type Value = [String; 2]; }

/// The URLParser statically checks if the Request is conforming to the API structure-wise
/// and will return a MalformedRequest Response if not.
pub struct URLParser;
impl BeforeMiddleware for URLParser
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        let path: Vec<String> = req.url.path.clone();

        if path.len() < 2 || path.len() > 3
        {
            return Err(IronError::new(MalformedRequest, status::BadRequest));
        }
        if path[0] == "" || path[1] == ""
        {
            return Err(IronError::new(MalformedRequest, status::BadRequest));
        }
        if path.len() == 3 && path[2] != ""
        {
            return Err(IronError::new(MalformedRequest, status::BadRequest));
        }

        // URL Structure gets added into req.extensions
        let controller: String = path[0].clone();
        let model: String = path[1].clone();

        let req_url: [String; 2] = [controller, model];
        req.extensions.insert::<URL>(req_url);

        Ok(())
    }
}
