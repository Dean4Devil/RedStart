use iron::prelude::*;
use iron::{BeforeMiddleware, Error};

use url::{Url, SchemeData};

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
        // Currently do precisely nothing
        println!("{}", req);
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
	let p = Url::parse(req.url.to_string().as_slice()).ok();
	println!("{}", p);
	if(p.unwrap().path().unwrap()[0] != "".to_string())
	{
		return false;
	}

	let mut found = false;
	let u = p.unwrap();
	let q = u.query_pairs().unwrap();

	for x in q.iter()
	{
		let r = "r".to_string();
		let val = match *x
		{
			(r, value) => value,
			// (_, value) => {continue;},
		};

		if(!val.contains("/"))
		{
			return false;
		}
		else
		{
			found = true;
		}
	}
	
	if(found != true)
	{
		return false;
	}

	true
}