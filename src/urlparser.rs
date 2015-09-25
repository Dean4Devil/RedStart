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
impl Key for URL { type Value = (String, Option<String>, Option<String>); }

/// The URLParser statically checks if the Request is conforming to the API structure-wise
/// and will return a MalformedRequest Response if not.
pub struct URLParser;
impl BeforeMiddleware for URLParser
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        let collection: String;
        let element: Option<String>;
        let field: Option<String>;

        let mut path = req.url.path.clone();

        if path.len() < 1 || path.len() > 3
        {
            return Err(IronError::new(MalformedRequest, status::BadRequest));
        }

        collection = path[0].clone();
        element = match path.get_mut(1)
        {
            Some(e) => Some(e.clone()),
            None => None,
        };
        field = match path.get_mut(2)
        {
            Some(e) => Some(e.clone()),
            None => None,
        };

        let url_parts: (String, Option<String>, Option<String>) = (collection, element, field);
        req.extensions.insert::<URL>(url_parts);

        Ok(())
    }
}
