use iron::prelude::*;
use iron::{BeforeMiddleware, Error};
use iron::typemap::Assoc;

use queryst;
use serialize::json;

// Errors for the win!
#[deriving(Show)]
pub struct MalformedRequest;
#[deriving(Show)]
pub struct NotFound;

impl Error for MalformedRequest
{
    fn name(&self) -> &'static str { "MalformedRequest" }
}

impl Error for NotFound
{
    fn name(&self) -> &'static str { "NoRoute" }
}

// The actual URLParser struct
pub struct URLParser;

impl Assoc<&'static [&'static str]> for URLParser
{

}

// Make URLParser a BeforeMiddleware
impl BeforeMiddleware for URLParser
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        if(check_url(req).is_ok())
        {
        	Ok(())
        }
        else
        {
        	Err(box MalformedRequest as IronError)
        }
    }
}

fn check_url(req: &mut Request) -> Result<(), ()>
{
    let url2 = req.url.clone();
    if url2.path == vec!["".to_string()]
    {
        // query type String
        let mut query = match url2.query
        {
            Some(e) => e,
            None => return Err(()),
        };

        // qs type &str
        let qs = query.as_slice();

        // query_json type Json
        let mut query_json = match queryst::parse(qs)
        {
            Ok(e) => e.clone(),
            Err(_) => return Err(()),
        };

        // ... does not live long enough ...
        let mut route_json = match query_json.find("r")
        {
            Some(e) => e.clone(),
            None => return Err(()),
        };

        let mut route_string = match route_json.as_string()
        {
            Some(e) => e,
            None => return Err(()),
        };

        if route_string.contains("/")
        {
            let mut route_it = route_string.split('/');
            let mut route_vec = vec![route_it.next().unwrap().clone(), route_it.next().unwrap().clone()];
            req.extensions.insert::<URLParser, &[&str]>(route_vec.as_slice().clone());
            return Ok(())
        }
    }

    Err(())

}
