use iron::prelude::*;
use iron::{BeforeMiddleware, Error};

use url::{Url, SchemeData};
use queryst;

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

impl URLParser
{

}

// Make URLParser a BeforeMiddleware
impl BeforeMiddleware for URLParser
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        if(check_url(req))
        {
        	Ok(())
        }
        else
        {
        	Err(box MalformedRequest as IronError)
        }
    }
}

fn check_url(req: &mut Request) -> bool
{
	let parsed = Url::parse(req.url.to_string().as_slice()).ok().unwrap();
    {
    	let path = parsed.path();
    	if(path.unwrap()[0] != "".to_string() || path.unwrap().len() != 1)
    	{
    		return false;
    	}
    }

    println!("{}", parsed.query);
    if parsed.query.is_none()
    {
        return false;
    }

    let qs = queryst::parse(parsed.query.unwrap().as_slice());
    println!("{}", qs);

    return false

}
